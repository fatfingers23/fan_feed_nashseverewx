# Fan made Nashville Severe Weather news feed

This is a *FAN* made news feed for the Nashville Severe Weather website. All content and thanks belongs to
[Nashville Severe Weather](https://nashvillesevereweather.com/).

### Why?

Nashville Severe Weather is a great resource for weather updates in the Nashville area, and with that they get a lot of
traffic.
With this traffic I noticed the site can take a while to load sometimes, so i wanted to take a crack at making a version
that
caches the info and loads it a bit faster.

### How?

Nashville Severe Weather appears to be a WordPres site, all this project does is makes a web
request to the WP api and caches it in redis for 1 hour. If someone else visits the website with in that hour
it ups the time to another hour. Redis cache is a in memory cache to allow for rapid access to the article data.

### API Details

`GET /api/recent-posts` - Returns the most recent 10 posts from the Nashville Severe Weather website.

`GET /api/clear-cache` - Clears the redis cache so you can get the most recent posts and compare the speed the site
loads.