# whitebox-exec
#Code wasn't finished 


**/src** contains client code written in javascript.
client code is meant to glue together interfaces and code and pass it to the executor, receive result and proof, show it to the user.
to run: 
```
node src/main.js
``` 

**/rustcoderunner** contains codes of attempted implementation code execution on risk0 to run rust code.
rustcoderunner starts an API server listening to **localhost:4000**, accpeting **/run** and **/compile** requests

to run:
```
cd rustcoderunner
cargo run --release
```

