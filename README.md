# Clientless
A RotMG clientless bot, written in Rust

I wrote this mostly to learn rust and how it works, as I already have a feature packed clientless written in Go. This project will likely be left as-is.
This bot has a decent amount of functionality and can do a fair amount of tasks. Ways to extend this would be to get projectiles working as well as loading xml gamedata.

Another way of extending this would be to ditch the 1:2 threading model that it uses. (Each client runs on its own thread, and spins up another thread for the networking stuff). Either an async approach and/or some sort of threadpool approach would probably be the best bet for the networking side of things.


