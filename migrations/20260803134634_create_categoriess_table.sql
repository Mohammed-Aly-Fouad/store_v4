-- ========================================================
-- 1. Helper Functions
-- ========================================================

-- تحديث updated_at تلقائيًا
CREATE OR REPLACE FUNCTION public.update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- دالة تطبيع النص (نفس الموجودة في migration الماركات، مكررة هنا بأمان لأنها CREATE OR REPLACE)
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

-- الحد الأقصى لعمق التصنيفات (3 مستويات)
CREATE OR REPLACE FUNCTION public.check_category_depth()
RETURNS TRIGGER AS $$
DECLARE
    current_depth INT := 1;
    parent_cursor BIGINT := NEW.parent_id;
BEGIN
    -- منع التصنيف من أن يكون أبًا لنفسه
    IF NEW.id IS NOT NULL AND NEW.id = NEW.parent_id THEN
        RAISE EXCEPTION 'Category cannot be its own parent.';
    END IF;

    -- الصعود في شجرة الآباء
    WHILE parent_cursor IS NOT NULL LOOP
        current_depth := current_depth + 1;

        IF current_depth > 3 THEN
            RAISE EXCEPTION 'Category depth limit exceeded (Maximum 3 levels allowed).';
        END IF;

        SELECT parent_id INTO parent_cursor
        FROM public.categories
        WHERE id = parent_cursor;
    END LOOP;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ========================================================
-- 2. Table: public.categories
-- ========================================================

DROP TABLE IF EXISTS public.categories CASCADE;

CREATE TABLE public.categories
(
    id          bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    parent_id   bigint,
    name_en     character varying(255) NOT NULL,
    name_ar     character varying(255) NOT NULL,
    notes       text,
    created_at  timestamp with time zone NOT NULL DEFAULT now(),
    updated_at  timestamp with time zone NOT NULL DEFAULT now(),

    CONSTRAINT categories_pkey PRIMARY KEY (id),
    CONSTRAINT fk_categories_parent FOREIGN KEY (parent_id)
        REFERENCES public.categories (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT,

    -- منع الأسماء الفارغة أو المسافات فقط
    CONSTRAINT categories_name_en_not_blank CHECK (btrim(name_en) <> ''),
    CONSTRAINT categories_name_ar_not_blank CHECK (btrim(name_ar) <> '')
);

ALTER TABLE IF EXISTS public.categories OWNER TO mohammed;

-- ========================================================
-- 3. Indexes for categories
-- ========================================================

CREATE INDEX idx_categories_parent_id
    ON public.categories (parent_id);

-- منع تكرار الاسم الإنجليزي داخل نفس الأب (بعد التطبيع)
CREATE UNIQUE INDEX idx_categories_unique_name_en_parent
    ON public.categories (normalize_text(name_en), parent_id)
    NULLS NOT DISTINCT;

-- منع تكرار الاسم العربي داخل نفس الأب (إلكترونيات = الكترونيات)
CREATE UNIQUE INDEX idx_categories_unique_name_ar_parent
    ON public.categories (normalize_text(name_ar), parent_id)
    NULLS NOT DISTINCT;

-- (اختياري) لتسريع البحث بـ LIKE 'xxx%'
CREATE INDEX idx_categories_search_name_ar
    ON public.categories (normalize_text(name_ar) text_pattern_ops);

CREATE INDEX idx_categories_search_name_en
    ON public.categories (normalize_text(name_en) text_pattern_ops);

-- ========================================================
-- 4. Triggers for categories
-- ========================================================

DROP TRIGGER IF EXISTS enforce_category_depth ON public.categories;
CREATE TRIGGER enforce_category_depth
    BEFORE INSERT OR UPDATE ON public.categories
    FOR EACH ROW
    EXECUTE FUNCTION public.check_category_depth();

DROP TRIGGER IF EXISTS update_categories_modtime ON public.categories;
CREATE TRIGGER update_categories_modtime
    BEFORE UPDATE ON public.categories
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();