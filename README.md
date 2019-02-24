## Blood-Sucker ##

This is just an attempt to learn rust with a project that seems like a good fit: 
an [MQTT](http://mqtt.org/documentation) implementation.

This is very much the half-blind stumblings of someone interested in rust-lang.

### TODO
* Serde implmentation for VariableHeader
* Serde implmentation for Payload
* Deserialization implementation for Message
* Connect `Sessions::handle_message` with a `TcpListener` and deserialization
  (in other words, write the actual server loop)
