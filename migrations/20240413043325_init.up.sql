-- CREATE PRIMARY SCHEMAS
create table accounts (
	id uuid not null default gen_random_uuid(),
	email varchar(150) not null unique,
	username varchar(16) not null unique,
	password bytea not null,
	about varchar(255),
	is_admin boolean not null default false,
	primary key (id)
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
	content varchar(8000),
	created_by uuid,
	created_at timestamp not null default current_timestamp,
	primary key (id),
	foreign key (created_by) references accounts(id)
);

create table comments (
	id uuid default gen_random_uuid(),
	content varchar(8000),
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
	content varchar(8000),
	created_at timestamp not null default current_timestamp,
	primary key (created_by, for_post),
	foreign key (created_by) references accounts(id),
	foreign key (for_post) references posts(id)
);

create table comment_upvotes(
	id uuid default gen_random_uuid(),
	created_by uuid,
	for_comment uuid,
	content varchar(8000),
	created_at timestamp not null default current_timestamp,
	primary key (created_by, for_comment),
	foreign key (created_by) references accounts(id),
	foreign key (for_comment) references comments(id)
);
