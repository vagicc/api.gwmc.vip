
-- 司法拍卖车爬虫表
CREATE TABLE lawsuit_reptile(
  "id" SERIAL PRIMARY KEY,
  "title" CHARACTER VARYING(255) NOT NULL,
  "list_img" CHARACTER VARYING(255) DEFAULT NULL,

  "price_base" MONEY NOT NULL DEFAULT 0,
  "current_price" MONEY NOT NULL DEFAULT 0,
  "assess_price" MONEY NOT NULL DEFAULT 0,
  "margin" MONEY NOT NULL DEFAULT 0,

  "start_time" TIMESTAMP WITHOUT time ZONE,
  "end_time" TIMESTAMP WITHOUT time ZONE,
 
  "lng" decimal DEFAULT NULL,
  "lat" decimal DEFAULT NULL,
  "address" CHARACTER VARYING(250) DEFAULT NULL,
  "disposal_unit" CHARACTER VARYING(255) DEFAULT NULL,
  "external_url" CHARACTER VARYING(255) DEFAULT NULL,
  "belong" SMALLINT DEFAULT NULL,
  "stage" CHARACTER VARYING(8) DEFAULT NULL,
  "status" SMALLINT NOT NULL DEFAULT 1,
  "push" BOOLEAN DEFAULT FALSE,
  "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);
CREATE INDEX idx_lawsuit_reptile_status ON  lawsuit_reptile USING btree(status);
COMMENT ON TABLE lawsuit_reptile IS '司法拍卖车爬虫表';
COMMENT ON COLUMN lawsuit_reptile.title IS '车标题';
COMMENT ON COLUMN lawsuit_reptile.list_img IS '封面图-列表图';
COMMENT ON COLUMN lawsuit_reptile.price_base IS '起拍价';
COMMENT ON COLUMN lawsuit_reptile.current_price IS '当前价';
COMMENT ON COLUMN lawsuit_reptile.assess_price IS '评估价';
COMMENT ON COLUMN lawsuit_reptile.margin IS '保证金';
COMMENT ON COLUMN lawsuit_reptile.start_time IS '开始时间';
COMMENT ON COLUMN lawsuit_reptile.end_time IS '结束时间';
COMMENT ON COLUMN lawsuit_reptile.lng IS '坐标:经度';
COMMENT ON COLUMN lawsuit_reptile.lat IS '坐标:纬度';
COMMENT ON COLUMN lawsuit_reptile.address IS '地址';
COMMENT ON COLUMN lawsuit_reptile.disposal_unit IS '处置单位:所属法院';
COMMENT ON COLUMN lawsuit_reptile.external_url IS '拍卖详情URL';
COMMENT ON COLUMN lawsuit_reptile.belong IS '所属平台（1.淘宝、2.京东）';
COMMENT ON COLUMN lawsuit_reptile.stage IS '拍卖阶段（一拍、二拍、变卖、撤回）';
COMMENT ON COLUMN lawsuit_reptile.status IS '状态（1待开拍、2竞拍中、已结束:3成交，4流拍、0无效或撤回）';
COMMENT ON COLUMN lawsuit_reptile.push IS '是否推送';
COMMENT ON COLUMN lawsuit_reptile.create_time IS '创建时间';

-- 爬虫相册记录表
CREATE TABLE lawsuit_reptile_photo(
    "lrpid" SERIAL PRIMARY KEY,
    "lrid" INTEGER NOT NULL,
    "external_small" CHARACTER VARYING(255) DEFAULT NULL,
    "external_middle" CHARACTER VARYING(255) DEFAULT NULL,
    "external_original" CHARACTER VARYING(255) DEFAULT NULL,
    "front_cover" BOOLEAN DEFAULT FALSE
);
CREATE INDEX idx_lawsuit_reptile_photo_laid ON lawsuit_reptile_photo USING btree(lrid);
COMMENT ON TABLE lawsuit_reptile_photo IS '爬虫司法拍卖相册表';
COMMENT ON COLUMN lawsuit_reptile_photo.lrid IS '司法拍卖机动车表(lawsuit_reptile)ID';
COMMENT ON COLUMN lawsuit_reptile_photo.external_small IS '外链小图';
COMMENT ON COLUMN lawsuit_reptile_photo.external_middle IS '外链中图';
COMMENT ON COLUMN lawsuit_reptile_photo.external_original IS '外链原图';
COMMENT ON COLUMN lawsuit_reptile_photo.front_cover IS '是否为封面图';

