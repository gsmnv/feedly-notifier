# Feedly notifier

![No Updates](https://i.imgur.com/sPWm6Ag.png)
![Some Updates](https://i.imgur.com/gUZBv58.png)
![Updates](https://i.imgur.com/bwhpx8g.png)

Small tray icon which will indicate when you've got some update in your feedly
account.
Refreshes once in a 10 minutes and has 3 different colors for different
unread articles:

Color | Unread count
---|---
Green | 0
Yellow | \< 50
Red | > 50

When you hover your mouse pointer over the icon then it'll display a tool-tip
with exact number of unread articles in your categories.

## Installation
Currently compiles only on nightly Rust compiler version.
```
cargo install feedly-notifier
```

## Usage
You can either run it in foreground with `feedly-notifier` or background
with `feedly-notifier &`. To exit just press your right mouse button on
the icon and click `exit` button. Make sure you've got `FEEDLY_ACCESS_TOKEN`
environment variable set, you can obtain required token by following these
[instructions](https://developer.feedly.com/v3/developer/)
