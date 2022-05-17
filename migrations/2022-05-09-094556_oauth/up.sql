-- 
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
