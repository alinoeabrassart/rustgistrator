Clone of gliderlab's Registrator using rust

What the code does
------------------

Simple things:
- list all the container running when the program starts
- listen to all containers that start

- on every detected container run inspection to detect interesting tags
or labels
- create a description that fit consul api

- send the description to consul to fit with the state of the node
(running containers...)
