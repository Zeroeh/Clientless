# Clientless
A RotMG clientless bot, written in Rust

I wrote this mostly to learn rust and how it works, as I already have a feature packed clientless written in Go. This project will likely be left as-is, unless I get super bored one day.
This bot has a decent amount of functionality and can do a fair amount of tasks. Ways to extend this would be to get projectiles working as well as loading xml gamedata.

Another way of extending this would be to ditch the 1:2 threading model that it uses. (Each client runs on its own thread, and spins up another thread for the networking stuff). Either an async approach and/or some sort of threadpool approach would probably be the best bet for the networking side of things.

# How to install and run
Please note that I wrote this on linux so windows/macos users, you will kinda have to figure things out on your own.

Open a terminal, begin by cloning the repo:
- ``git clone https://github.com/Zeroeh/Clientless.git && cd Clientless``

Now make the build scripts executable:
- ``chmod +x debug.sh && chmod +x release.sh``

Now, go into the config/settings.json and change the build version if needed. In the config/accounts.json file, add your bots email and password to their respective fields. Change the game server ip as you see fit. Also make sure your charid matches up.

Once everything seems good, run the build script which will compile and run the bots:
``./debug.sh`` or, for release mode: ``./release.sh``

Everything should start compiling and if compiled successfully, should get a message that your bot joined the game:

``<bots email> joined Nexus! ObjectID: 50194``

Also, there seems to be some nasty bug that causes clients to reach 100% CPU usage on the main thread when reconnecting sometimes. Not sure why this happens. I suspect it could be some sort of deadlock? If you know or experience it, let me know as I'm quite curious.
