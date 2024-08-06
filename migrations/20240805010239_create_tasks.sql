create table if not exists tasks
(
	id	integer primary key not null,
	description	text not null,
	completed_at	datetime,
	created_at	datetime not null
)
