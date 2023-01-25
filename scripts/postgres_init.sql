drop table if exists users;
create table if not exists users (
    id uuid default gen_random_uuid() primary key,
    username varchar(32) not null default '',
    password varchar(256) not null default '',
    fund_password varchar(256) not null default '',
    login_count int not null default 0,
    login_last_at bigint not null default 0,
    created bigint not null default 0,
    updated bigint not null default 0
);
create unique index on users(username);

SELECT col_description(a.attrelid,a.attnum) as comment,
    format_type(a.atttypid,a.atttypmod) as type,
    a.attname as name, 
    a.attnotnull as notnull
FROM pg_class as c, pg_attribute as a 
where c.relname = 'users' and a.attrelid = c.oid and a.attnum > 0;