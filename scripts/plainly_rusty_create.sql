CREATE TYPE post_status AS ENUM ('PRIVATE', 'PUBLIC', 'DELETED');

CREATE TYPE comment_status AS ENUM ('PUBLIC', 'DELETED');

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    content TEXT NOT NULL,
    views INT DEFAULT 0,
    summary TEXT,
    status post_status DEFAULT 'PUBLIC' :: post_status,
    CONSTRAINT unique_title_content UNIQUE (title, content)
);

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE post_tags (
    post_id INT,
    tag_id INT,
    PRIMARY KEY (post_id, tag_id),
    FOREIGN KEY (post_id) REFERENCES posts(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id),
    CONSTRAINT unique_post_tag UNIQUE (post_id, tag_id)
);

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    parent_id INT,
    post_id INT,
    github_id INT,
    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    content TEXT NOT NULL,
    status comment_status DEFAULT 'PUBLIC' :: comment_status,
    FOREIGN KEY (parent_id) REFERENCES comments(id),
    FOREIGN KEY (post_id) REFERENCES posts(id),
    CONSTRAINT unique_user_comment UNIQUE (post_id, github_id, content)
);

---

INSERT INTO
    tags (name)
VALUES
    ('Technology'),
    ('Programming'),
    ('Database');

INSERT INTO
    posts (title, content, summary)
VALUES
    (
        'Getting Started with PostgreSQL',
        'PostgreSQL is a powerful open-source relational database...',
        'Learn the basics of PostgreSQL.'
    ),
    (
        'Web Development Best Practices',
        'Follow these best practices for building scalable and maintainable web applications...',
        'Improve your web development skills.'
    );

INSERT INTO
    post_tags (post_id, tag_id)
VALUES
    (1, 1),
    (1, 2),
    (2, 2),
    (2, 3);

INSERT INTO
    comments (post_id, github_id, content)
VALUES
    (1, 583231, 'Great introduction!'),
    (1, 86649490, 'Looking forward to more posts.'),
    (2, 583231, 'These best practices are helpful!');

INSERT INTO
    comments (parent_id, post_id, github_id, content)
VALUES
    (1, 1, 86649490, 'Thanks!'),
    (4, 2, 583231, 'Glad you found them helpful!');