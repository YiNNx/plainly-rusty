-- Create the Tags table
CREATE TABLE Tags (
    tid SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL
);

-- Create the Posts table
CREATE TABLE Posts (
    pid SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    content TEXT NOT NULL,
    views INT DEFAULT 0,
    summary TEXT,
    status VARCHAR(7) DEFAULT 'PUBLIC' CHECK (status IN ('PRIVATE', 'PUBLIC', 'DELETED'))
);

-- Create the PostTags table
CREATE TABLE PostTags (
    post_id INT,
    tag_id INT,
    PRIMARY KEY (post_id, tag_id),
    FOREIGN KEY (post_id) REFERENCES Posts(pid),
    FOREIGN KEY (tag_id) REFERENCES Tags(tid)
);

-- Create the Users table
CREATE TABLE Users (
    uid SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    role VARCHAR(5) DEFAULT 'USER' CHECK (role IN ('USER', 'OWNER'))
);

-- Create the Comments table
CREATE TABLE Comments (
    cid SERIAL PRIMARY KEY,
    parent_cid INT,
    post_id INT,  
    username VARCHAR(255),
    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    content TEXT NOT NULL,
    status VARCHAR(7) DEFAULT 'PUBLIC' CHECK (status IN ('PRIVATE', 'PUBLIC', 'DELETED')),
    FOREIGN KEY (parent_cid) REFERENCES Comments(cid),
    FOREIGN KEY (post_id) REFERENCES Posts(pid), 
    FOREIGN KEY (username) REFERENCES Users(username)
);


---

INSERT INTO Users (username, role) VALUES ('YiNNx', 'OWNER');

INSERT INTO Tags (name) VALUES ('Tag1');
INSERT INTO Tags (name) VALUES ('Tag2');

INSERT INTO Posts (title, content, summary, status) VALUES ('Post1 Title', 'Post1 Content', 'Post1 Summary', 'PUBLIC');
INSERT INTO Posts (title, content, summary, status) VALUES ('Post2 Title', 'Post2 Content', 'Post2 Summary', 'PRIVATE');

INSERT INTO PostTags (post_id, tag_id) VALUES (1, 1);
INSERT INTO PostTags (post_id, tag_id) VALUES (1, 2);
INSERT INTO PostTags (post_id, tag_id) VALUES (2, 2);

INSERT INTO Users (username, role) VALUES ('User1', 'USER');
INSERT INTO Users (username, role) VALUES ('User2', 'USER');

INSERT INTO Comments (parent_cid,post_id, username, content, status) VALUES (NULL,1,  'User1','Comment1 Content', 'PUBLIC');
INSERT INTO Comments (parent_cid,post_id, username, content, status) VALUES (1,1, 'User2',1, 'Comment2 Content', 'PRIVATE');