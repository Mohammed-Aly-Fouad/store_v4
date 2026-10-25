-- Add migration script here
-- ========================================================
-- 0. Helper Functions
-- ========================================================

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

-- ========================================================
-- 1. Table: public.product_variants
-- ========================================================

DROP TABLE IF EXISTS public.product_variants CASCADE;

CREATE TABLE public.product_variants
(
    id                 bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    product_id         bigint NOT NULL,
    brand_id           bigint NOT NULL,
    name_ar            character varying(255) NOT NULL,
    name_en            character varying(255) NOT NULL,
    sku                character varying(100) NOT NULL,
    barcode            character varying(100),
    shelf_location     character varying(100),
    stock_quantity     integer NOT NULL DEFAULT 0,
    reorder_threshold  integer NOT NULL DEFAULT 0,
    is_active          boolean NOT NULL DEFAULT true,
    attr               jsonb NOT NULL DEFAULT '{}'::jsonb,
    notes              text,
    created_at         timestamp with time zone NOT NULL DEFAULT now(),
    updated_at         timestamp with time zone NOT NULL DEFAULT now(),

    CONSTRAINT product_variants_pkey PRIMARY KEY (id),
    CONSTRAINT fk_product_variants_product FOREIGN KEY (product_id)
        REFERENCES public.products (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT,
    CONSTRAINT fk_product_variants_brand FOREIGN KEY (brand_id)
        REFERENCES public.brands (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT,

    CONSTRAINT check_stock_quantity_non_negative CHECK (stock_quantity >= 0),
    CONSTRAINT check_reorder_threshold_non_negative CHECK (reorder_threshold >= 0),

    -- منع القيم الفارغة أو المسافات فقط
    CONSTRAINT product_variants_name_en_not_blank CHECK (btrim(name_en) <> ''),
    CONSTRAINT product_variants_name_ar_not_blank CHECK (btrim(name_ar) <> ''),
    CONSTRAINT product_variants_sku_not_blank CHECK (btrim(sku) <> ''),
    CONSTRAINT product_variants_barcode_not_blank CHECK (barcode IS NULL OR btrim(barcode) <> '')
);

ALTER TABLE IF EXISTS public.product_variants OWNER TO mohammed;

-- ========================================================
-- 2. Foreign Key Indexes
-- ========================================================

CREATE INDEX idx_product_variants_product_id
    ON public.product_variants (product_id);

CREATE INDEX idx_product_variants_brand_id
    ON public.product_variants (brand_id);

-- ========================================================
-- 3. Unique Indexes (بعد التطبيع)
-- ========================================================

-- SKU فريد (حروف كبيرة/صغيرة ومسافات لا تفرق)
CREATE UNIQUE INDEX idx_product_variants_unique_sku
    ON public.product_variants (normalize_text(sku));

-- الباركود فريد عند وجوده
CREATE UNIQUE INDEX idx_product_variants_unique_barcode
    ON public.product_variants (btrim(barcode))
    WHERE barcode IS NOT NULL;

-- الاسم الإنجليزي فريد داخل نفس المنتج
CREATE UNIQUE INDEX idx_product_variants_unique_name_en_product
    ON public.product_variants (product_id, normalize_text(name_en));

-- الاسم العربي فريد داخل نفس المنتج (أحمر = احمر، مدرّسة = مدرسه)
CREATE UNIQUE INDEX idx_product_variants_unique_name_ar_product
    ON public.product_variants (product_id, normalize_text(name_ar));

-- (اختياري) لتسريع البحث بـ LIKE 'xxx%'
CREATE INDEX idx_product_variants_search_name_ar
    ON public.product_variants (normalize_text(name_ar) text_pattern_ops);

CREATE INDEX idx_product_variants_search_name_en
    ON public.product_variants (normalize_text(name_en) text_pattern_ops);

-- ========================================================
-- 4. JSONB & Partial Active Indexes
-- ========================================================

CREATE INDEX idx_product_variants_attr_gin
    ON public.product_variants USING gin (attr);

CREATE INDEX idx_product_variants_active_product
    ON public.product_variants (product_id)
    WHERE is_active = true;

-- ========================================================
-- 5. Trigger
-- ========================================================

DROP TRIGGER IF EXISTS update_product_variants_modtime ON public.product_variants;
CREATE TRIGGER update_product_variants_modtime
    BEFORE UPDATE ON public.product_variants
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();