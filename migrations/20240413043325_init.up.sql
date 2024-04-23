-- Add up migration script here

-- CREATE PRIMARY SCHEMAS
create table accounts (
	id uuid not null default gen_random_uuid(),
	email varchar(150) not null unique,
	password bytea not null,
	username varchar(16) not null unique,
	about varchar(255),
	primary key (id)
);

create table roles (
	id uuid not null default gen_random_uuid() unique,
	name varchar(16) not null unique,
	description varchar(255) not null
);

create table permissions (
	id uuid not null default gen_random_uuid() unique,
	name varchar(16) not null unique,
	description varchar(255) not null
);

create table accounts_roles (
	account_id uuid,
	role_id uuid,
	primary key (account_id, role_id),
	foreign key (account_id) references accounts(id),
	foreign key (role_id) references roles(id)
);

create table roles_permissions (
	role_id uuid,
	permissions_id uuid,
	primary key (role_id, permissions_id),
	foreign key (role_id) references roles(id),
	foreign key (permissions_id) references permissions(id)
);

create table startups (
	id uuid default gen_random_uuid(),
	created_by uuid,
	owned_by uuid,
	name varchar(64) not null,
	about varchar(255),
	created_at timestamp not null default current_timestamp,
	primary key (id),
	foreign key (created_by) references accounts(id),
	foreign key (owned_by) references accounts(id)
);

create table posts (
	id uuid default gen_random_uuid(),
	content text,
	created_by uuid,
	created_at timestamp not null default current_timestamp,
	primary key (id),
	foreign key (created_by) references accounts(id)
);

create table comments (
	id uuid default gen_random_uuid(),
	content text,
	created_by uuid,
	created_at timestamp not null default current_timestamp,
	primary key (id),
	foreign key (created_by) references accounts(id)
);

-- CREATE JOINT SCHEMAS (POST_UPVOTES, COMMENT_UPVOTES)
create table post_upvotes (
	id uuid default gen_random_uuid(),
	created_by uuid,
	for_post uuid,
	created_at timestamp not null default current_timestamp,
	primary key (created_by, for_post),
	foreign key (created_by) references accounts(id),
	foreign key (for_post) references posts(id)
);

create table comment_upvotes(
	id uuid default gen_random_uuid(),
	created_by uuid,
	for_comment uuid,
	created_at timestamp not null default current_timestamp,
	primary key (created_by, for_comment),
	foreign key (created_by) references accounts(id),
	foreign key (for_comment) references comments(id)
);
