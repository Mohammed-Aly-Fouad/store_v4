-- Add migration script here
-- ========================================================
-- Insert Product Variants Seed Data
-- ========================================================
INSERT INTO public.product_variants (product_id, brand_id, name_en, name_ar, stock_quantity) VALUES 
(
  (SELECT id FROM public.products WHERE name_en = 'Glue stick'),
  (SELECT id FROM public.brands WHERE name_en = 'PASCO'),
  'Pasco Glue Stick 8g', 'جلو ستيك باسك 8 جرام', 29
),
(
  (SELECT id FROM public.products WHERE name_en = 'Mechanical Pencil'),
  (SELECT id FROM public.brands WHERE name_en = 'FABER CASTLE'),
  'Faber-Castell 2122 2B Pencil', 'قلم رصاص فابر كاستل 2122 2B', 72
),
(
  (SELECT id FROM public.products WHERE name_en = 'Pocket Notebook'),
  (SELECT id FROM public.brands WHERE name_en = 'EVERGREEN'),
  'Everygreen Modeling Clay 12 Colors', 'صلصال افيفرجرين 12 لون', 1
),
(
  (SELECT id FROM public.products WHERE name_en = 'Sketchbook'),
  (SELECT id FROM public.brands WHERE name_en = 'EVERGREEN'),
  'Everygreen Rubber Bands Red', 'أساتك افيفرجرين أحمر', 10
),
(
  (SELECT id FROM public.products WHERE name_en = 'Sketchbook'),
  (SELECT id FROM public.brands WHERE name_en = 'EVERGREEN'),
  'Everygreen Rubber Bands Green', 'أساتك افيفرجرين أخضر', 13
),
(
  (SELECT id FROM public.products WHERE name_en = 'Bulldog Clip'),
  (SELECT id FROM public.brands WHERE name_en = 'FLOWER'),
  'Flower Stamp Pad Ink Blue 30ml', 'حبر ختامة فلاور أزرق 30 مل', 9
),
(
  (SELECT id FROM public.products WHERE name_en = 'Protractor'),
  (SELECT id FROM public.brands WHERE name_en = 'ETMAN GROUP'),
  'Otima Glue Gun Stick Large Clear', 'غيار مسدس شمع أوتيما كبير شفاف', 27
),
(
  (SELECT id FROM public.products WHERE name_en = 'Thermal Paper Roll'),
  (SELECT id FROM public.brands WHERE name_en = 'POWER'),
  'Power Price Labels 1 Line Roll', 'ملصقات أسعار باور 1 خط بكرة', 10
),
(
  (SELECT id FROM public.products WHERE name_en = 'Legal Contract'),
  (SELECT id FROM public.brands WHERE name_en = 'ETMAN GROUP'),
  'Otima Wrapping Ribbon 4cm Multi-color', 'شريط لف هدايا أوتيما 4 سم متعدد الألوان', 15
),
(
  (SELECT id FROM public.products WHERE name_en = 'U-Shape Sheet Protector'),
  (SELECT id FROM public.brands WHERE name_en = 'MIDGO'),
  'M&G Pocket Display Book A4 10 Pockets', 'هولدر جيوب ام اند جي A4 10 جيوب', 5
),
(
  (SELECT id FROM public.products WHERE name_en = 'U-Shape Sheet Protector'),
  (SELECT id FROM public.brands WHERE name_en = 'MIDGO'),
  'M&G Pocket Display Book A4 20 Pockets', 'هولدر جيوب ام اند جي A4 20 جيب', 3
),
(
  (SELECT id FROM public.products WHERE name_en = 'U-Shape Sheet Protector'),
  (SELECT id FROM public.brands WHERE name_en = 'MIDGO'),
  'M&G Pocket Display Book A4 30 Pockets', 'هولدر جيوب ام اند جي A4 30 جيب', 3
),
(
  (SELECT id FROM public.products WHERE name_en = 'U-Shape Sheet Protector'),
  (SELECT id FROM public.brands WHERE name_en = 'DL_DINGLI'),
  'Deli Pocket Display Book A4 40 Pockets', 'هولدر جيوب ديلي A4 40 جيب', 3
),
(
  (SELECT id FROM public.products WHERE name_en = 'Eraser'),
  (SELECT id FROM public.brands WHERE name_en = 'EVERGREEN'),
  'Everygreen Correction Pen 6ml', 'قلم كوريكتور افيفرجرين 6 مل', 12
),
(
  (SELECT id FROM public.products WHERE name_en = 'Eraser'),
  (SELECT id FROM public.brands WHERE name_en = 'EVERGREEN'),
  'Everygreen Correction Pen 8ml', 'قلم كوريكتور افيفرجرين 8 مل', 10
),
(
  (SELECT id FROM public.products WHERE name_en = 'Clipboard'),
  (SELECT id FROM public.brands WHERE name_en = 'ELUCKY_EVERLUCKY'),
  'El-Keshky Bulldog Clip 25mm Black', 'مشبك ورق الكشكي 25 مم أسود', 144
),
(
  (SELECT id FROM public.products WHERE name_en = 'Rubber Bands'),
  (SELECT id FROM public.brands WHERE name_en = 'SOFI PLAST'),
  'Safari Report Cover A4 Prong Multi-color', 'دوسيه تقارير سفاري A4 بنز متعدد الألوان', 25
),
(
  (SELECT id FROM public.products WHERE name_en = 'Sticky Note'),
  (SELECT id FROM public.brands WHERE name_en = 'SEMA'),
  'Samir & Aly Clipboard A4 Single', 'بلانشيطة سمير وعلى A4 مفرد', 6
),
(
  (SELECT id FROM public.products WHERE name_en = 'Sticky Note'),
  (SELECT id FROM public.brands WHERE name_en = 'BRAVO'),
  'Bravo Clipboard A4 with Pen Holder', 'بلانشيطة برافو A4 بالحامل والقلم', 4
),
(
  (SELECT id FROM public.products WHERE name_en = 'Sticky Note'),
  (SELECT id FROM public.brands WHERE name_en = 'DIGITAL'),
  'Digital Clipboard A4 PVC Coated', 'بلانشيطة ديجيتال A4 كرتون مضغوط', 2
),
(
  (SELECT id FROM public.products WHERE name_en = 'L-Shape Sheet Protector'),
  (SELECT id FROM public.brands WHERE name_en = 'SOFI PLAST'),
  'Safari U-Shape Protector A4 95 Micron', 'حافظة حرف يو سفاري A4 95 مايكرون', 46
),
(
  (SELECT id FROM public.products WHERE name_en = 'L-Shape Sheet Protector'),
  (SELECT id FROM public.brands WHERE name_en = 'EVERGREEN'),
  'Everygreen U-Shape Protector A4 Light', 'حافظة حرف يو افيفرجرين A4 خفيفة', 25
),
(
  (SELECT id FROM public.products WHERE name_en = 'Graphite Pencil'),
  (SELECT id FROM public.brands WHERE name_en = 'DOMS'),
  'Doms Single Hole Pencil Sharpener Small', 'براية دومس فتحة واحدة صغيرة', 100
),
(
  (SELECT id FROM public.products WHERE name_en = 'Highlighter'),
  (SELECT id FROM public.brands WHERE name_en = 'ROTO'),
  'Rotring Permanent Marker Blue Chisel', 'قلم ماركر دائم روتريتو أزرق مشطوف', 21
),
(
  (SELECT id FROM public.products WHERE name_en = 'Highlighter'),
  (SELECT id FROM public.brands WHERE name_en = 'ROTO'),
  'Rotring Permanent Marker Red Chisel', 'قلم ماركر دائم روتريتو أحمر مشطوف', 24
),
(
  (SELECT id FROM public.products WHERE name_en = 'Highlighter'),
  (SELECT id FROM public.brands WHERE name_en = 'ROTO'),
  'Rotring Permanent Marker Black Chisel', 'قلم ماركر دائم روتريتو أسود مشطوف', 12
),
(
  (SELECT id FROM public.products WHERE name_en = 'Pen'),
  (SELECT id FROM public.brands WHERE name_en = 'MICRO'),
  'Micro Mechanical Pencil 0.7mm', 'قلم رصاص سنون مايكرو 0.7 مم', 23
),
(
  (SELECT id FROM public.products WHERE name_en = 'Pocket Display Book'),
  (SELECT id FROM public.brands WHERE name_en = 'FABER CASTLE'),
  'Faber-Castell Eraser Red Small 189577', 'استيكة فابر كاستل صغيرة أحمر 189577', 30
),
(
  (SELECT id FROM public.products WHERE name_en = 'Pocket Display Book'),
  (SELECT id FROM public.brands WHERE name_en = 'DOMS'),
  'Doms Eraser Dust Free White Small', 'استيكة دومس خالية من الغبار أبيض صغيرة', 17
),
(
  (SELECT id FROM public.products WHERE name_en = 'Compass'),
  (SELECT id FROM public.brands WHERE name_en = 'QUEEN'),
  'Queen Protractor 180 Degree Clear', 'منقلة كوين 180 درجة شفاف', 36
),
(
  (SELECT id FROM public.products WHERE name_en = 'Lease agreement'),
  (SELECT id FROM public.brands WHERE name_en = 'MICHAEL'),
  'Mitchell Metal Ruler 30cm Heavy', 'مسطرة معدن ميتشيل 30 سم ثقيلة', 30
),
(
  (SELECT id FROM public.products WHERE name_en = 'Lease agreement'),
  (SELECT id FROM public.brands WHERE name_en = 'CHANYI'),
  'Chuangyi Plastic Ruler 30cm Clear', 'مسطرة بلاستيك تشوانجي 30 سم شفاف', 24
),
(
  (SELECT id FROM public.products WHERE name_en = 'Report Cover'),
  (SELECT id FROM public.brands WHERE name_en = 'ALWARRAK'),
  'World Thermal Paper Roll 57mm White', 'بكر حراري ورلد 57 مم أبيض', 28
),
(
  (SELECT id FROM public.products WHERE name_en = 'Pencil Case'),
  (SELECT id FROM public.brands WHERE name_en = 'GAOERJIEFU'),
  'Gaole Metallic Balloon 10 Inch 20 Pcs', 'بالون ميتاليك جول 10 بوصة 20 قطعة', 5
),
(
  (SELECT id FROM public.products WHERE name_en = 'Lease agreement'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Grid Ruler 20cm Clear', 'مسطرة مربعات بريما 20 سم شفاف', 22
),
(
  (SELECT id FROM public.products WHERE name_en = 'Lease agreement'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Grid Ruler 30cm Clear', 'مسطرة مربعات بريما 30 سم شفاف', 22
),
(
  (SELECT id FROM public.products WHERE name_en = 'Modeling Clay'),
  (SELECT id FROM public.brands WHERE name_en = 'ROTO'),
  'Rotring Highlighter Bright 4 Colors Pack', 'طقم أقلام هاي لايتر روتريتو 4 ألوان', 2
),
(
  (SELECT id FROM public.products WHERE name_en = 'Mechanical Pencil'),
  (SELECT id FROM public.brands WHERE name_en = 'DELI'),
  'Deli Triangular Pencil 2B 37013', 'قلم رصاص ديلي مثلث 2B 37013', 45
),
(
  (SELECT id FROM public.products WHERE name_en = 'Wrapping Ribbon'),
  (SELECT id FROM public.brands WHERE name_en = 'QUEEN'),
  'Queen Venn Stencil Ruler 20cm', 'مسطرة أشكال فين كوين 20 سم', 24
),
(
  (SELECT id FROM public.products WHERE name_en = 'Pen'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Mechanical Pencil 0.5mm Stand', 'قلم رصاص سنون بريما 0.5 مم', 4
),
(
  (SELECT id FROM public.products WHERE name_en = 'Pen'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Mechanical Pencil 0.7mm Stand', 'قلم رصاص سنون بريما 0.7 مم', 12
),
(
  (SELECT id FROM public.products WHERE name_en = 'Pen'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Mechanical Pencil 0.9mm Stand', 'قلم رصاص سنون بريما 0.9 مم', 8
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'ROTO'),
  'Rotring Liquid Ball Pen 0.7mm Blue', 'قلم جاف ليكويد روتريتو 0.7 مم أزرق', 21
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'ROTO'),
  'Rotring Liquid Ball Pen 0.7mm Red', 'قلم جاف ليكويد روتريتو 0.7 مم أحمر', 10
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'ROTO'),
  'Rotring Liquid Ball Pen 0.7mm Black', 'قلم جاف ليكويد روتريتو 0.7 مم أسود', 22
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PENSAN'),
  'Pentel My-Tech Pen 0.7mm Blue', 'قلم جاف بينتل ماي تيك 0.7 مم أزرق', 12
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PENSAN'),
  'Pentel My-Tech Pen 0.7mm Green', 'قلم جاف بينتل ماي تيك 0.7 مم أخضر', 12
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PENSAN'),
  'Pentel Triball Pen 1.0mm Blue', 'قلم جاف بينتل ترايبال 1.0 مم أزرق', 11
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Forma Ball Pen 1.0mm Blue', 'قلم جاف بريما فورما 1.0 مم أزرق', 16
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Solo Needle Point Pen 0.7mm Blue', 'قلم جاف بريما سولو سن إبرة 0.7 مم أزرق', 20
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Lino Ball Pen 1.0mm Blue', 'قلم جاف بريما لينو 1.0 مم أزرق', 8
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Nova Ball Pen 0.7mm Black', 'قلم جاف بريما نوفا 0.7 مم أسود', 10
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Forsa Ball Pen 0.7mm Blue', 'قلم جاف بريما فورسا 0.7 مم أزرق', 9
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Magro Ball Pen 0.7mm Blue', 'قلم جاف بريما ماجرو 0.7 مم أزرق', 9
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Tri-touch Ball Pen 0.7mm Blue', 'قلم جاف بريما تراي تاتش 0.7 مم أزرق', 11
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Genta Ball Pen 0.7mm Blue', 'قلم جاف بريما جينتا 0.7 مم أزرق', 5
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'ROTO'),
  'Rotring Easy Flow Liquid Pen 1.0mm Red', 'قلم إيزي فلو روتريتو 1.0 مم أحمر', 21
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima 25 Ball Pen 0.7mm Blue', 'قلم جاف بريما 25-0.7 مم أزرق', 7
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'ROTO'),
  'Rotring G-Plus Liquid Pen 0.7mm Blue', 'قلم جاف جي بلس روتريتو 0.7 مم أزرق', 12
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'ROTO'),
  'Rotring Glase Liquid Pen 0.7mm Blue', 'قلم جاف جليز روتريتو 0.7 مم أزرق', 3
),
(
  (SELECT id FROM public.products WHERE name_en = 'Pen'),
  (SELECT id FROM public.brands WHERE name_en = 'MAPED'),
  'Maped Mechanical Pencil 0.5mm', 'قلم رصاص سنون مابيد 0.5 مم', 2
),
(
  (SELECT id FROM public.products WHERE name_en = 'Ruler'),
  (SELECT id FROM public.brands WHERE name_en = 'DOMS'),
  'Doms Metric Ruler 20cm Plastic', 'مسطرة دومس 20 سم بلاستيك', 10
),
(
  (SELECT id FROM public.products WHERE name_en = 'Modeling Clay'),
  (SELECT id FROM public.brands WHERE name_en = 'DU HU'),
  'Dahle Highlighter Yellow Chisel', 'قلم هاي لايتر داهلي أصفر مشطوف', 10
),
(
  (SELECT id FROM public.products WHERE name_en = 'Permanent Marker'),
  (SELECT id FROM public.brands WHERE name_en = 'ROTO'),
  'Rotring Whiteboard Marker Blue Board 501', 'قلم سبورة روتريتو أزرق 501', 12
),
(
  (SELECT id FROM public.products WHERE name_en = 'Permanent Marker'),
  (SELECT id FROM public.brands WHERE name_en = 'ROTO'),
  'Rotring Whiteboard Marker Black Board 501', 'قلم سبورة روتريتو أسود 501', 12
),
(
  (SELECT id FROM public.products WHERE name_en = 'Permanent Marker'),
  (SELECT id FROM public.brands WHERE name_en = 'ROTO'),
  'Rotring Whiteboard Marker Red Board 501', 'قلم سبورة روتريتو أحمر 501', 12
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Oro Ball Pen 0.7mm Blue', 'قلم جاف بريما أورو 0.7 مم أزرق', 7
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Fancy & Rosa Ball Pen 0.7mm Blue', 'قلم جاف بريما فانسي وروسا 0.7 مم أزرق', 12
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Bronzo Ball Pen 0.7mm Blue', 'قلم جاف بريما برونزو 0.7 مم أزرق', 10
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Gely Fine Point Pen 0.5mm Blue', 'قلم جيل بريما جيلي 0.5 مم أزرق', 20
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Gely Fine Point Pen 0.5mm Black', 'قلم جيل بريما جيلي 0.5 مم أسود', 10
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Gely Fine Point Pen 0.5mm Red', 'قلم جيل بريما جيلي 0.5 مم أحمر', 10
),
(
  (SELECT id FROM public.products WHERE name_en = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name_en = 'PRIMA'),
  'Prima Fino Ball Pen 0.7mm Black', 'قلم جاف بريما فينو 0.7 مم أسود', 10
),
(
  (SELECT id FROM public.products WHERE name_en = 'Mechanical Pencil'),
  (SELECT id FROM public.brands WHERE name_en = 'POWER'),
  'Power Maden Pencil 2B with Eraser', 'قلم رصاص باور مدن 2B بالمسطرة والاستيكة', 36
),
(
  (SELECT id FROM public.products WHERE name_en = 'Plastic Envelope with Button'),
  (SELECT id FROM public.brands WHERE name_en = 'GAZELLE'),
  'Ghazala DL Envelope Gummed 80g White', 'ظرف غزالة DL صمغ 80 جرام أبيض', 150
),
(
  (SELECT id FROM public.products WHERE name_en = 'Plastic Envelope with Button'),
  (SELECT id FROM public.brands WHERE name_en = 'ELNOUR'),
  'Ghazala DL Envelope Self-Seal 80g White', 'ظرف غزالة DL ذاتي اللصق 80 جرام أبيض', 40
),
(
  (SELECT id FROM public.products WHERE name_en = 'Plastic Envelope with Button'),
  (SELECT id FROM public.brands WHERE name_en = 'GAZELLE'),
  'Ghazala C5 Envelope Self-Seal 100g White', 'ظرف غزالة C5 ذاتي اللصق 100 جرام أبيض', 150
),
(
  (SELECT id FROM public.products WHERE name_en = 'Plastic Envelope with Button'),
  (SELECT id FROM public.brands WHERE name_en = 'GAZELLE'),
  'Ghazala C4 Envelope Self-Seal 100g White', 'ظرف غزالة C4 ذاتي اللصق 100 جرام أبيض', 50
),
(
  (SELECT id FROM public.products WHERE name_en = 'Plastic Envelope with Button'),
  (SELECT id FROM public.brands WHERE name_en = 'AMIRA FACTORY ALEX'),
  'Amir DL Envelope Air Gummed 80g White', 'ظرف طيران الأمير DL صمغ 80 جرام أبيض', 50
),
(
  (SELECT id FROM public.products WHERE name_en = 'Zipper Document Wallet'),
  (SELECT id FROM public.brands WHERE name_en = 'EVERGREEN'),
  'Everygreen Plastic Envelope Button FC 209-10', 'حافظة كبسولة بلاستيك افيفرجرين FC 209-10', 25
),
(
  (SELECT id FROM public.products WHERE name_en = 'Zipper Document Wallet'),
  (SELECT id FROM public.brands WHERE name_en = 'EVERGREEN'),
  'Everygreen Plastic Envelope Button FC 209-14', 'حافظة كبسولة بلاستيك افيفرجرين FC 209-14', 90
),
(
  (SELECT id FROM public.products WHERE name_en = 'Zipper Document Wallet'),
  (SELECT id FROM public.brands WHERE name_en = 'EVERGREEN'),
  'Everygreen Plastic Envelope Button FC 209-1-18C', 'حافظة كبسولة بلاستيك افيفرجرين FC 209-1-18C', 20
),
(
  (SELECT id FROM public.products WHERE name_en = 'Stamp Pad Refill Ink'),
  (SELECT id FROM public.brands WHERE name_en = 'ELSAFA'),
  'Safa Sketchbook 17x24cm 18 Sheets 70g', 'كراس رسم الصفا 17*24 سم 18 ورقة 70 جرام', 12
),
(
  (SELECT id FROM public.products WHERE name_en = 'Stamp Pad Refill Ink'),
  (SELECT id FROM public.brands WHERE name_en = 'GHARIB'),
  'El-Gharbawi Sketchbook 17x24cm 16 Sheets 60g', 'كراس رسم الغرباوي 17*24 سم 16 ورقة 60 جرام', 40
),
(
  (SELECT id FROM public.products WHERE name_en = 'Glue stick'),
  (SELECT id FROM public.brands WHERE name_en = 'PASCO'),
  'Pasco Glue Stick 8g (Duplicate)', 'جلو ستيك باسك 8 جرام (مكرر)', 29
),
(
  (SELECT id FROM public.products WHERE name_en = 'Mechanical Pencil'),
  (SELECT id FROM public.brands WHERE name_en = 'FABER CASTLE'),
  'Faber-Castell 2122 2B Pencil (Duplicate)', 'قلم رصاص فابر كاستل 2122 2B (مكرر)', 72
),
(
  (SELECT id FROM public.products WHERE name_en = 'Zipper Document Wallet'),
  (SELECT id FROM public.brands WHERE name_en = 'EVERGREEN'),
  'Everygreen Plastic Envelope FC (Duplicate)', 'حافظة كبسولة بلاستيك افيفرجرين (مكرر)', 20
),
(
  (SELECT id FROM public.products WHERE name_en = 'Whiteboard Marker'),
  (SELECT id FROM public.brands WHERE name_en = 'FABER CASTLE'),
  'Faris Registration Folder Blue', 'ملف تقديم الفارس أزرق', 180
);