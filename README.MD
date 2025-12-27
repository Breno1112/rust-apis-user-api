## Objetivo do projeto

API de usuários simples em container que é capaz de se comunicar com mongoDB e Redis

* Endpoint de Health Check - OK
* Endpoint de consulta de usuários
    * Consultar primeiro no Redis. Se não existir, consultar no MongoDB e salvar no Redis com TTL de 5 minutos
* Endpoint de cadastro de usuários
* Endpoint de alteração de usuários
* Endpoint de remoção de usuários

* Tudo feito em containers
* API expondo endpoints em SSL
* Container non-keepalive que apenas gera os certificados para o MongoDB, o Redis e a API.
* Os 3 certificados devem ser diferentes
* A API deve confiar nos certificados do Redis e do MongoDB