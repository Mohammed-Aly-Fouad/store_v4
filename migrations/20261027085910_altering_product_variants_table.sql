-- Add migration script here
ALTER TABLE product_variants 
ADD COLUMN attr JSONB NOT NULL DEFAULT '{}'::jsonb;

-- Optional: Create a GIN index if you plan to filter/search by attributes frequently
CREATE INDEX idx_product_variants_attr ON product_variants USING gin (attr);