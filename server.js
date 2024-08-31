const ws = require('websocket').server;
const http = require('http');
async function main()
{
    const port = '2208'
    
    const server = http.createServer();
    server.listen(port,()=>
        {
            console.log(`listening on port ${port}`);
            console.log('waiting for incoming connections')
        });
    const wss = new ws(
        {
            httpServer: server,
        });
    wss.on('request',(req)=>
        {
            const conn = req.accept(null,req.origin);
            console.log('Connection Established');
            conn.on('close',()=>
                {
                    console.log('Connection closed');
                });
            conn.on('message',(msg)=>
                {
                    console.log(msg.utf8Data);
                    wss.broadcast(msg.utf8Data);
                });
        });
}
main();