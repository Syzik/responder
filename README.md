# Responder

## Features

* Every URL points to the same controller, no matter the path or filename
* Content-Type is automatically set based on file extension
* Accept settings from both POST and GET bodies
* `?preview` to see the raw HTTP response in a code block

## TODO

* Some query string format to easily set response 
  * headers
    * CORS
    * content-type
    * custom
  * body data
    * raw
    * length to generate cyclic
  * response code
* Fetch body content from URL, but safe
  * maybe resolve DNS, check range, and perform request on IP + Host header. will need to disable HTTPS checking
