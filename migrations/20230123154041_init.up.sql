CREATE TABLE IF NOT EXISTS users (
    uid varchar(10) NOT NULL,
    birthday TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    sex varchar(1) NOT NULL,
    name varchar(50) NOT null,
    PRIMARY KEY (uid)
);