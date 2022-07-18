-- 跟我买车 start


-- 车辆推荐 lawsuit_autocar

--  法拍详情页：

--   推荐文字说明

-- 法拍车文章推荐lawsuit_autocar_article
CREATE TABLE lawsuit_autocar_article(
  "laid" SERIAL PRIMARY KEY REFERENCES lawsuit_autocar(id),
  "article_content" TEXT DEFAULT NULL,
  "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);
COMMENT ON TABLE lawsuit_autocar_article IS '视频详情表';
COMMENT ON COLUMN lawsuit_autocar_article.laid IS '司法拍卖机动车表ID';
COMMENT ON COLUMN lawsuit_autocar_article.article_content IS '文章内容';

-- 车辆相册表
CREATE TABLE lawsuit_autocar_photo(
    "lapid" SERIAL PRIMARY KEY,
    "laid" INTEGER NOT NULL,

    "external_small" CHARACTER VARYING(255) DEFAULT NULL,
    "external_middle" CHARACTER VARYING(255) DEFAULT NULL,
    "external_original" CHARACTER VARYING(255) DEFAULT NULL,

    "small" CHARACTER VARYING(255) DEFAULT NULL,
    "middle" CHARACTER VARYING(255) DEFAULT NULL,
    "original" CHARACTER VARYING(255) DEFAULT NULL,
    "path" CHARACTER VARYING(255) DEFAULT NULL,
    "title" CHARACTER VARYING(58) DEFAULT NULL,
    "extension" CHARACTER VARYING(8) DEFAULT NULL,
    "type" CHARACTER VARYING(18) DEFAULT NULL,
    "front_cover" BOOLEAN DEFAULT FALSE,
    "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);
CREATE INDEX idx_lawsuit_autocar_photo_laid ON lawsuit_autocar_photo USING btree(laid);
COMMENT ON TABLE lawsuit_autocar_photo IS '司法拍卖机动车相册表';
COMMENT ON COLUMN lawsuit_autocar_photo.laid IS '司法拍卖机动车表(lawsuit_autocar)ID';
COMMENT ON COLUMN lawsuit_autocar_photo.external_small IS '外链小图';
COMMENT ON COLUMN lawsuit_autocar_photo.external_middle IS '外链中图';
COMMENT ON COLUMN lawsuit_autocar_photo.external_original IS '外链原图';
COMMENT ON COLUMN lawsuit_autocar_photo.small IS '小图';
COMMENT ON COLUMN lawsuit_autocar_photo.middle IS '中图';
COMMENT ON COLUMN lawsuit_autocar_photo.original IS '原图';
COMMENT ON COLUMN lawsuit_autocar_photo.path IS '图片路径';
COMMENT ON COLUMN lawsuit_autocar_photo.title IS '图片名不带扩展';
COMMENT ON COLUMN lawsuit_autocar_photo.extension IS '图片扩展名如：jpg';
COMMENT ON COLUMN lawsuit_autocar_photo.type IS '图片类型如：image/jpeg';
COMMENT ON COLUMN lawsuit_autocar_photo.front_cover IS '是否为封面图';
-- 车辆视频表

CREATE TABLE lawsuit_autocar(
  "id" SERIAL PRIMARY KEY,
  "acid" INTEGER DEFAULT NULL,
  "title" CHARACTER VARYING(255) NOT NULL,
  "summary" CHARACTER VARYING(255) NOT NULL,
  "list_img" CHARACTER VARYING(255) DEFAULT NULL,
  "visit" bigint NOT NULL DEFAULT 0,

  "price_base" MONEY NOT NULL DEFAULT 0,
  "current_price" MONEY NOT NULL DEFAULT 0,
  "assess_price" MONEY NOT NULL DEFAULT 0,
  "margin" MONEY NOT NULL DEFAULT 0,
  "recommended_price" MONEY NOT NULL DEFAULT 0,

  "start_time" TIMESTAMP WITHOUT time ZONE,
  "end_time" TIMESTAMP WITHOUT time ZONE,
  "recommend" SMALLINT NOT NULL DEFAULT 1,
 
  "lng" decimal DEFAULT NULL,
  "lat" decimal DEFAULT NULL,
  "address" CHARACTER VARYING(250) DEFAULT NULL,
  "disposal_unit" CHARACTER VARYING(255) DEFAULT NULL,
  "external_url" CHARACTER VARYING(255) DEFAULT NULL,
  "belong" SMALLINT DEFAULT NULL,
  "stage" CHARACTER VARYING(8) DEFAULT NULL,
  "status" SMALLINT NOT NULL DEFAULT 1,
  "show" BOOLEAN DEFAULT TRUE,
  "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);
CREATE INDEX idx_lawsuit_autocar_acid ON lawsuit_autocar USING btree(acid);
CREATE INDEX idx_lawsuit_autocar_status ON lawsuit_autocar USING btree(status);
COMMENT ON TABLE lawsuit_autocar IS '司法拍卖机动车表';
COMMENT ON COLUMN lawsuit_autocar.acid IS '车辆分类表ID';
COMMENT ON COLUMN lawsuit_autocar.title IS '车标题';
COMMENT ON COLUMN lawsuit_autocar.summary IS '车摘要';
COMMENT ON COLUMN lawsuit_autocar.list_img IS '封面图-列表图';
COMMENT ON COLUMN lawsuit_autocar.visit IS '浏览次数';
COMMENT ON COLUMN lawsuit_autocar.price_base IS '起拍价';
COMMENT ON COLUMN lawsuit_autocar.current_price IS '当前价';
COMMENT ON COLUMN lawsuit_autocar.assess_price IS '评估价';
COMMENT ON COLUMN lawsuit_autocar.margin IS '保证金';
COMMENT ON COLUMN lawsuit_autocar.recommended_price IS '最高推荐价';
COMMENT ON COLUMN lawsuit_autocar.start_time IS '开始时间';
COMMENT ON COLUMN lawsuit_autocar.end_time IS '结束时间';
COMMENT ON COLUMN lawsuit_autocar.recommend IS '推荐星数1-10';
COMMENT ON COLUMN lawsuit_autocar.lng IS '坐标:经度';
COMMENT ON COLUMN lawsuit_autocar.lat IS '坐标:纬度';
COMMENT ON COLUMN lawsuit_autocar.address IS '地址';
COMMENT ON COLUMN lawsuit_autocar.disposal_unit IS '处置单位:所属法院';
COMMENT ON COLUMN lawsuit_autocar.external_url IS '拍卖详情URL';
COMMENT ON COLUMN lawsuit_autocar.belong IS '所属平台（1.淘宝、2.京东）';
COMMENT ON COLUMN lawsuit_autocar.stage IS '拍卖阶段（一拍、二拍、变卖、撤回）';
COMMENT ON COLUMN lawsuit_autocar.status IS '状态（1待开拍、2竞拍中、已结束:3成交，4流拍、0无效或撤回）';
COMMENT ON COLUMN lawsuit_autocar.show IS '是否展示';
COMMENT ON COLUMN lawsuit_autocar.create_time IS '创建时间';

-- 车辆类型
CREATE TABLE lawsuit_autocar_category(
    "acid" SERIAL PRIMARY KEY,
    "cname" CHARACTER VARYING(50),
    "parent_id" INTEGER NOT NULL DEFAULT 0,
    "level" SMALLINT DEFAULT 1,
    "seo_title" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_keywords" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_description" CHARACTER VARYING(255) DEFAULT NULL,
    "order_by" SMALLINT NOT NULL,
    "is_show" boolean DEFAULT TRUE
);
CREATE INDEX idx_lawsuit_autocar_category_parent_id ON lawsuit_autocar_category USING btree(parent_id);
CREATE INDEX idx_lawsuit_autocar_category_level ON lawsuit_autocar_category USING btree(level);
COMMENT ON TABLE lawsuit_autocar_category IS '车辆分类表';
COMMENT ON COLUMN lawsuit_autocar_category.acid IS '分类ID';
COMMENT ON COLUMN lawsuit_autocar_category.cname IS '分类名称';
COMMENT ON COLUMN lawsuit_autocar_category.parent_id IS '父分类ID';
COMMENT ON COLUMN lawsuit_autocar_category.level IS '类别层级';
COMMENT ON COLUMN lawsuit_autocar_category.seo_title IS 'SEO标题';
COMMENT ON COLUMN lawsuit_autocar_category.seo_keywords IS 'SEO关键词';
COMMENT ON COLUMN lawsuit_autocar_category.seo_description IS 'SEO描述';
COMMENT ON COLUMN lawsuit_autocar_category.order_by IS '排序:小排前，大排后';
COMMENT ON COLUMN lawsuit_autocar_category.is_show IS '是否显示：默认true显示，flase不显示';

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

-- 跟我买车 end

-- postgres oauth2  start
CREATE TABLE oauth_access_tokens(
  "access_token" CHARACTER(38) PRIMARY KEY,
  "client_id" CHARACTER(30) NOT NULL,
  "user_id" INTEGER NOT NULL,
  "expires" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp(),
  "scope" CHARACTER VARYING(80) DEFAULT NULL
);
CREATE INDEX idx_oauth_access_tokens_client_id ON oauth_access_tokens USING btree(client_id);
CREATE INDEX idx_oauth_access_tokens_user_id ON oauth_access_tokens USING btree(user_id);
COMMENT ON TABLE oauth_access_tokens IS 'token表';
COMMENT ON COLUMN oauth_access_tokens.access_token IS 'token，也是主键';
COMMENT ON COLUMN oauth_access_tokens.client_id IS '登录客户端';
COMMENT ON COLUMN oauth_access_tokens.user_id IS '用户ID';
COMMENT ON COLUMN oauth_access_tokens.expires IS '有效时间';

CREATE TABLE oauth_refresh_tokens (
  "refresh_token" CHARACTER(38) PRIMARY KEY,
  "client_id" CHARACTER(30) NOT NULL,
  "user_id" INTEGER NOT NULL,
  "expires" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp(),
  "scope" CHARACTER VARYING(80) DEFAULT NULL
);
CREATE INDEX idx_oauth_refresh_tokens_client_id ON oauth_refresh_tokens USING btree(client_id);
CREATE INDEX idx_oauth_refresh_tokens_user_id ON oauth_refresh_tokens USING btree(user_id);
COMMENT ON TABLE oauth_refresh_tokens IS 'oauth_refresh_tokens表';
COMMENT ON COLUMN oauth_refresh_tokens.refresh_token IS '刷新token，也是主键';
COMMENT ON COLUMN oauth_refresh_tokens.client_id IS '登录客户端';
COMMENT ON COLUMN oauth_refresh_tokens.user_id IS '用户ID';
COMMENT ON COLUMN oauth_refresh_tokens.expires IS '有效时间';


CREATE TABLE oauth_authorization_codes(
  "authorization_code" CHARACTER(48) PRIMARY KEY,
  "client_id" CHARACTER(30) NOT NULL,
  "user_id" INTEGER NOT NULL,
  "redirect_uri" CHARACTER VARYING(200) DEFAULT NULL,
  "expires" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp(),
  "scope" CHARACTER VARYING(80) DEFAULT NULL,
  "id_token" CHARACTER VARYING(80) DEFAULT NULL
);
CREATE INDEX idx_oauth_authorization_codes_client_id ON oauth_authorization_codes USING btree(client_id);
CREATE INDEX idx_oauth_authorization_codes_user_id ON oauth_authorization_codes USING btree(user_id);
COMMENT ON TABLE oauth_authorization_codes IS 'oauth_authorization_codes表';

CREATE TABLE oauth_clients(
  "client_id" CHARACTER(30) PRIMARY KEY,
  "client_secret" CHARACTER VARYING(80) DEFAULT NULL,
  "redirect_uri" CHARACTER VARYING(200) DEFAULT NULL,
  "grant_types" CHARACTER VARYING(80) DEFAULT NULL,
  "scope" CHARACTER VARYING(80) DEFAULT NULL,
  "user_id" INTEGER DEFAULT NULL
);
CREATE INDEX idx_oauth_clients_user_id ON oauth_clients USING btree(user_id);
COMMENT ON TABLE oauth_clients IS 'oauth_clients表';
COMMENT ON COLUMN oauth_clients.client_id IS '主键';
COMMENT ON COLUMN oauth_clients.user_id IS '用户ID,这个字段多余，可去掉';

INSERT INTO oauth_clients 
(client_id, client_secret, redirect_uri, grant_types, scope, user_id) VALUES
('android', 'androidtestsecret', NULL, 'password refresh_token', 'app', NULL),
('ios', 'androidtestsecret', NULL, 'password refresh_token', 'app', NULL),
('linux', 'androidtestsecret', NULL, 'password refresh_token', 'deskop', NULL);

CREATE TABLE oauth_jwt (
  "client_id" CHARACTER(30) PRIMARY KEY,
  "subject" CHARACTER VARYING(80) DEFAULT NULL,
  "public_key" CHARACTER VARYING(200) DEFAULT NULL
);
COMMENT ON TABLE oauth_jwt IS 'oauth_jwt表';
COMMENT ON COLUMN oauth_jwt.client_id IS '主键';

CREATE TABLE oauth_scopes (
  "scope" CHARACTER(80) PRIMARY KEY,
  "is_default" SMALLINT DEFAULT NULL
);
COMMENT ON TABLE oauth_scopes IS 'oauth_scopes表';

CREATE TABLE oauth_users(
    "user_id" SERIAL PRIMARY KEY,
    "username" CHARACTER VARYING(58) NOT NULL,
    "password" CHARACTER VARYING(40) NOT NULL,
    "salt" CHARACTER(10) DEFAULT NULL,
    "scope" CHARACTER VARYING(80) DEFAULT NULL,
    "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp(),
    "last_login" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()  
);
CREATE UNIQUE INDEX idx_oauth_users_username ON oauth_users USING btree(username);
COMMENT ON TABLE oauth_users IS '用户登录信息表';
COMMENT ON COLUMN oauth_users.username IS '登录用户名';

-- postgres oauth2  end

