Solução Rust para o problema do exercício utilizando diagonais principais e secundárias de uma matriz.

O programa se inicia declarando algumas variáveis: 
soma - variável que vai receber e armazenar a soma dos valores que precisamos somar.
o - variável para ler a entrada do usuário (se dejesa somar ou se deseja saber a média).
path - Cria um objeto referenciando o caminho do arquivo JSON que será lido.
file - Abre o arquivo especificado pelo caminho path para leitura
reader - Cria um BufReader para ler o arquivo de forma eficiente.

após a declaração das variáveis, o programa imprime a matriz e depois pede ao usuário qual operação deseja fazer.
O programa então, imprime A soma ou a média assim como foi requisitado pelo usuário, e armazena o resultado na variável o.

Após esta introdução do programa, se inicia então o processo para a soma dos números da área direita da matriz.

Neste programa, utilizei dois laços for, para percorrer linhas e colunas da matriz.
dentro do segundo laço for, impus uma condição if para saber se os números estão dentro da área direita da matriz.
Para saber isso, podemos utilizar diagonais de matrizes, lógica matemática para se utilizar em situações como esta.

A condição if usando o operador lógico "E", é a seguinte: 
if j(coluna) > i(linha) E j(coluna) > tamanho da matriz - i(linha) - 1

caso o número atenda a estes requisitos da condição, o número então é somado e armazenado na variável soma.
Além disso, optei por utilizar um print!("x "), para que, casos o número atenda a estas condições, será colocado a letra
x, e a matriz será impressa.

Caso o número não atenda a condição, será colocado um ".".

Depois de concluida a lógica e impressão da matriz, criei uma variável média, para que seja calculada a média.

E para finalizar uma estrutura condicional if, para receber a escolha do usuário e imprimir o resultado desejado.
