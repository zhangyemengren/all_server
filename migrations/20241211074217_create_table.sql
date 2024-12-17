-- 权限表，用于存储权限级别和描述
CREATE TABLE permissions
(
    id               SERIAL PRIMARY KEY,
    permission_level INT NOT NULL UNIQUE,
    description      TEXT
);


-- 创建用户表，包含账号、密码和权限外键关联
CREATE TABLE users
(
    id            SERIAL PRIMARY KEY,
    username      VARCHAR(50)  NOT NULL UNIQUE,
    password      VARCHAR(100) NOT NULL,
    permission_id INT          NOT NULL,
    FOREIGN KEY (permission_id) REFERENCES permissions (id)
);

