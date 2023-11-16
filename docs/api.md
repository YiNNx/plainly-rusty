# API Doc

## Basic Format:

- Address：`api.blog.just-plain.fun:3000 `
- API prefix：`/api/v1`    ( omitted below )

## Data Exchange Format:

- success:

```
{
    "success": true,
    "message": "",
    "hint": "",
    "data": {
        ... //any type
    }
}
```

- error:

```
{
    "success": false,
    "message": "error message",
    "hint": "debug info",	//only return at DEBUG mode
    "data": {}
}
```

Error will return an http status code other than `200`.

## Authorization

Authenticate with Json Web Token. Be like:

```
Authorization: Bearer TOKEN
```

APIs is divided into 2 parts. View & Comment part is available for everyone, while Post part needs Authorization. Sign-up is not opened publicly, and only the blog owner is able to sign in, post ,view the back-end data, etc.

## **Paginate**

Most of the `GET` requests support pagination fuction by using query params `skip` and `limit`.

For example:

`GET /post?skip=0&limit=10`

## API - View & Comment

#### Get Posts With Query Params (outline)

`GET /post`

`year` - specified year

`keyword` - search

`tag` - get posts by tag

`skip & limit `amount` - paginate

Response

```
{
	[	{
            "pid": 0,
            "status": 0,	//0-Public | 1-Private | 2-Script
            "title": "",
            "time": "2022-01-01 20:00:00",
            "tag":["",""],
            "excerpt": "",
            "stats": {
                "likes": 0,
                "views": 0,
                "comments": 0
            }
        },
        ...
    ]
}
```

If there is no authorization, only posts with `Public` status will be returned.

#### Get Post By Pid

`GET /post/<pid>`

Response

```
{
    "pid": 0,
    "status": 0,	//0-Public | 1-Private | 2-Script
    "type": 0,	//0-PlainText | 1-Markdown | 2-HTML
    "title": "",
    "time": "2022-01-01 20:00:00",
    "tag":["",""],
    "content": "",
    "stats": {
        "likes": 0,
        "views": 0,
        "comments": 0
    }
}
```

#### Get All Tags

`GET /tag`

```
{
	["TAG","TAG",...]
}
```

#### Like

`PUT /like/<pid>`

Request

```
{
	"status":true
}
```

#### Get Comments of Post

`GET /comment/<pid>`

Response

```
{
	[	{
            "cid": 1,
            "from": "",
            "from_url": "",
            "time": "2022-01-01 20:00:00",
            "content": "",
        },
        ...
    ]
}
```

#### Comment

`POST /comment/<pid>`

Request

```
{
	"from":"",
	"email":"",
	"from_url":"",
	"content":""
}
```

Response

```
{
	"cid":1
}
```

#### Sub Comment

`POST /comment/<pid>`

Request

```
{
	"from":"",
	"parent_cid":"",
	"email":"",
	"from_url":"",
	"content":""
}
```

Response

```
{
	"cid":1
}
```

#### Stats

`GET /stats`

Response

```
{
	"posts":0,
	"timestamp":0,
    "views": 0,
    "likes": 0,
    "comments": 0
}
```

### API - Post

*Token Needed

#### Log In

`GET /token?email=user@example.com&pwd=123456`

Response：

```
{
  "uid": 1,
  "token": "JWTTOKEN"
}
```

#### New Post

`POST /post`

Request

```
{
	"status":0,	//0-Public | 1-Private | 2-Script
	"type":0, //0-PlainText | 1-Markdown | 2-HTML
	"title":"title",
    "tags":["tag1","tag2"],
	"excerpt":"",
	"content":"content"
}
```

Response

```
{
	"pid":""
}
```

#### Update Post

`PUT /post/<pid>`

Request

```
{
	"status":0,	//0-Public | 1-Private | 2-Script
	"title":"title",
	"excerpt":"",
	"type":0, //0-PlainText | 1-Markdown | 2-HTML
	"content":"content",
    "tag":["tag1","tag2"]
}
```

#### Delete Post

`DELETE /post/<pid>`

Request

```
{
	"status":true
}
```

#### Detele Comment

`DELETE /comment/<cid>`

Request

```
{
	"status":true
}
```

#### Change Info

`PUT LikePost/info`

Request

```
{
	
}
```

