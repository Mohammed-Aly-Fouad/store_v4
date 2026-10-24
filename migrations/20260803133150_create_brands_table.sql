-- ========================================================
-- 0. Helper Functions
-- ========================================================

-- تحديث updated_at تلقائيًا
CREATE OR REPLACE FUNCTION public.update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- دالة تطبيع الاسم العربي:
--  أ إ آ -> ا   |   ى -> ي   |   ة -> ه
--  حذف التشكيل (الفتحة، الضمة، الكسرة، الشدة، السكون...) والتطويل (ـ)
--  تحويل الإنجليزي إلى lowercase
--  حذف المسافات الزائدة وتوحيد المسافات المتعددة
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
-- 1. Table: public.brands
-- ========================================================

DROP TABLE IF EXISTS public.brands CASCADE;

CREATE TABLE public.brands
(
    id          bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    name_en     character varying(255) NOT NULL,
    name_ar     character varying(255) NOT NULL,
    notes       text,
    created_at  timestamp with time zone NOT NULL DEFAULT now(),
    updated_at  timestamp with time zone NOT NULL DEFAULT now(),

    CONSTRAINT brands_pkey PRIMARY KEY (id),

    -- منع الأسماء الفارغة أو المسافات فقط
    CONSTRAINT brands_name_en_not_blank CHECK (btrim(name_en) <> ''),
    CONSTRAINT brands_name_ar_not_blank CHECK (btrim(name_ar) <> '')
);

ALTER TABLE IF EXISTS public.brands OWNER TO mohammed;

-- ========================================================
-- 2. Indexes for brands
-- ========================================================

-- منع التكرار في الاسم الإنجليزي (EVO = evo = Evo = " evo ")
CREATE UNIQUE INDEX idx_brands_unique_name_en
    ON public.brands (normalize_text(name_en));

-- منع التكرار في الاسم العربي (إيفو = ايفو = أيفو)
CREATE UNIQUE INDEX idx_brands_unique_name_ar
    ON public.brands (normalize_text(name_ar));

-- (اختياري) لتسريع البحث بـ LIKE 'xxx%'
CREATE INDEX idx_brands_search_name_ar
    ON public.brands (normalize_text(name_ar) text_pattern_ops);

CREATE INDEX idx_brands_search_name_en
    ON public.brands (normalize_text(name_en) text_pattern_ops);

-- ========================================================
-- 3. Trigger for brands
-- ========================================================

DROP TRIGGER IF EXISTS update_brands_modtime ON public.brands;
CREATE TRIGGER update_brands_modtime
    BEFORE UPDATE ON public.brands
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();