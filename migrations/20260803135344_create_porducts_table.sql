-- ========================================================
-- 0. Helper Functions
-- ========================================================

-- دوال مكررة بأمان (CREATE OR REPLACE) لتعمل هذه الـ migration وحدها

CREATE OR REPLACE FUNCTION public.update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

--  أ إ آ -> ا   |   ى -> ي   |   ة -> ه
--  حذف التشكيل والتطويل + lowercase + توحيد المسافات
CREATE OR REPLACE FUNCTION public.normalize_text(txt text)
RETURNS text AS $$
    SELECT btrim(
        regexp_replace(
            regexp_replace(
                translate(lower(txt), 'أإآىة', 'اااية'),
                '[\u064B-\u0652\u0640]', '', 'g'
            ),
            '\s+', ' ', 'g'
        )
    );
$$ LANGUAGE sql IMMUTABLE PARALLEL SAFE;

-- التأكد أن التصنيف المرتبط بالمنتج ليس تصنيفًا رئيسيًا (parent_id IS NULL)
CREATE OR REPLACE FUNCTION public.verify_product_category_is_leaf()
RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM public.categories
        WHERE id = NEW.category_id
          AND parent_id IS NULL
    ) THEN
        RAISE EXCEPTION 'Category ID % is a top-level category and cannot be assigned to products.', NEW.category_id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ========================================================
-- 1. Table: public.products
-- ========================================================

DROP TABLE IF EXISTS public.products CASCADE;

CREATE TABLE public.products
(
    id          bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    category_id bigint NOT NULL,
    name_en     character varying(255) NOT NULL,
    name_ar     character varying(255) NOT NULL,
    notes       text,
    created_at  timestamp with time zone NOT NULL DEFAULT now(),
    updated_at  timestamp with time zone NOT NULL DEFAULT now(),

    CONSTRAINT products_pkey PRIMARY KEY (id),
    CONSTRAINT fk_products_category FOREIGN KEY (category_id)
        REFERENCES public.categories (id)
        ON DELETE RESTRICT,

    -- منع الأسماء الفارغة أو المسافات فقط
    CONSTRAINT products_name_en_not_blank CHECK (btrim(name_en) <> ''),
    CONSTRAINT products_name_ar_not_blank CHECK (btrim(name_ar) <> '')
);

ALTER TABLE IF EXISTS public.products OWNER TO mohammed;

-- ========================================================
-- 2. Indexes for products
-- ========================================================

-- منع تكرار الاسم الإنجليزي (بعد التطبيع)
CREATE UNIQUE INDEX idx_products_unique_name_en
    ON public.products (normalize_text(name_en));

-- منع تكرار الاسم العربي (بعد التطبيع: أقلام = اقلام)
CREATE UNIQUE INDEX idx_products_unique_name_ar
    ON public.products (normalize_text(name_ar));

CREATE INDEX idx_products_category_id
    ON public.products (category_id);

-- (اختياري) لتسريع البحث بـ LIKE 'xxx%'
CREATE INDEX idx_products_search_name_ar
    ON public.products (normalize_text(name_ar) text_pattern_ops);

CREATE INDEX idx_products_search_name_en
    ON public.products (normalize_text(name_en) text_pattern_ops);

-- ========================================================
-- 3. Triggers for products
-- ========================================================

-- Trigger 1: تحديث updated_at تلقائيًا
DROP TRIGGER IF EXISTS update_products_modtime ON public.products;
CREATE TRIGGER update_products_modtime
    BEFORE UPDATE ON public.products
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();

-- Trigger 2: التحقق من مستوى التصنيف قبل الإدخال أو التعديل
DROP TRIGGER IF EXISTS check_product_category_before_save ON public.products;
CREATE TRIGGER check_product_category_before_save
    BEFORE INSERT OR UPDATE OF category_id ON public.products
    FOR EACH ROW
    EXECUTE FUNCTION public.verify_product_category_is_leaf();