CREATE TYPE post_status AS ENUM ('PRIVATE', 'PUBLIC', 'DELETED');

CREATE TYPE comment_status AS ENUM ('PUBLIC', 'DELETED');

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    content TEXT NOT NULL,
    summary TEXT,
    status post_status DEFAULT 'PUBLIC' :: post_status
);

CREATE UNIQUE INDEX unique_post_title ON posts (title) WHERE title <> 'DELETED';

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    CONSTRAINT unique_name UNIQUE (name)
);

CREATE TABLE post_tags (
    post_id INT NOT NULL,
    tag_id INT NOT NULL,
    PRIMARY KEY (post_id, tag_id),
    FOREIGN KEY (post_id) REFERENCES posts(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    parent_id INT,
    post_id INT NOT NULL,
    github_name TEXT NOT NULL,
    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    content TEXT NOT NULL,
    status comment_status DEFAULT 'PUBLIC' :: comment_status,
    FOREIGN KEY (parent_id) REFERENCES comments(id),
    FOREIGN KEY (post_id) REFERENCES posts(id),
    CONSTRAINT unique_user_comment UNIQUE (post_id, github_name, content)
);

--- SAMPLE DATA

-- INSERT INTO
--     tags (name)
-- VALUES
--     ('Technology'),
--     ('Programming'),
--     ('Database');

-- INSERT INTO
--     posts (title, content, summary, status)
-- VALUES
--     (
--         'Getting Started with Postgr1eSQL',
--         'PostgreSQL is a powerful open-source relational database...',
--         'Learn the basics of PostgreSQL.',
--         'PUBLIC'
--     ),
--     (
--         'Web Development Best Practices',
--         'Follow these best practices for building scalable and maintainable web applications...',
--         'Improve your web development ski2lls.',
--         'PUBLIC'
--     );

-- INSERT INTO
--     post_tags (post_id, tag_id)
-- VALUES
--     (1, 1),
--     (1, 2),
--     (2, 2),
--     (2, 3);
-- INSERT INTO
--     comments (post_id, github_name, content)
-- VALUES
--     (1, 'octocat', 'Great introduction!'),
--     (1, 'YiNNx', 'Looking forward to more posts.'),
--     (
--         2,
--         'octocat',
--         'These best practices are helpful!'
--     );
-- INSERT INTO
--     comments (parent_id, post_id, github_name, content)
-- VALUES
--     (1, 1, 'YiNNx', 'Thanks!'),
--     (4, 2, 'octocat', 'Glad you found them helpful!');