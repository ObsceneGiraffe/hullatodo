# Project Goal

Create a todo list for phone and desktop that is stored as a text file, and so can be edited by any text editor.
The todo list can be synced between devices using a peer-to-peer service such as Holochain.
The user can choose to remain completely anonymous

# Technical Goals

* Todos and config are stored as [text files|https://github.com/todotxt/todo.txt]
* Files are synced p2p (using Holochain?)
* No user information is needed to use or sync devices
* Start up time is as fast as possible
* The todos have no hierarchy, they only have tags
* Filtering and presenting the filtered todos should be as fast as possible, under 200ms if possible.
* the user ID should not 

# Account Creation

Upon first start the user is asked if they would like to create a new account or to link an existing one.

## Create a new account

The user is provided with a [unique ID|https://docs.rs/rand/0.5.0/rand/prng/index.html] 
The ID should be a base36 string as that can be easily written down if needed.

## Linking an existing account



# Sharing Filters


