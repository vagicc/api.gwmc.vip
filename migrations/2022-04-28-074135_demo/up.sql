-- 建表的SQL
CREATE TABLE demo(
    "id" SERIAL PRIMARY KEY,
    "name" CHARACTER VARYING(80) NOT NULL,
    "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);

-- 加索引
CREATE INDEX idx_demo_name ON demo USING btree(name);

COMMENT ON TABLE demo IS '示例表';

COMMENT ON COLUMN demo.id IS '主键ID';

COMMENT ON COLUMN demo.name IS '名字';

COMMENT ON COLUMN demo.create_time IS '创建时间';