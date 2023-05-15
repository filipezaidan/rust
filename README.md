# Linguagem de Progamação - Rust

<p  align="center">

<img  src="https://www.freecodecamp.org/news/content/images/2021/01/rust-mascot.png"  width="300">

</p>

Bem-vindo ao tuturial básico da linguagem Rust!

Nesta documentação, você as intruções básicas que precisa saber para começar a programar em Rust, desde a instalação, sintaxe básica até as características mais avançadas da linguagem. Esperamos que esta documentação ajude você a descobrir os benefícios e a elegância da programação em Rust.

---

## **_Tabela de conteúdo_**

1. [Contexto Histórico](#contexto-histórico)

1. [Características Gerais](#características-gerais)

1. [Instalação](#instalação)

1. [Sintaxe](#sintaxe)

- [Declação de Variáveis](#declaração-de-variáveis)

- [Como realizar um Hello World](#como-realizar-um-hello-world)

- [Estruturas de Condição](#estruturas-de-condição)

- [If/elseif/else](#ifelseifelse)

- [Switch](#switch)

- [Estruturas de Repetição](#estruturas-de-repetição)

- [For](#for)

- [While](#while)

- [Modularização (Funções)](#modularização-funções)

---

## Contexto Histórico

A linguagem Rust foi criada em 2006 por Graydon Hoare, com o objetivo de fornecer uma alternativa segura e de alto desempenho às linguagens C e C++. A Mozilla Research passou a patrocinar o projeto em 2010, e a primeira versão estável foi lançada em 2015.

Desde então, Rust tem se tornado cada vez mais popular entre desenvolvedores que trabalham com aplicativos de sistema e de alto desempenho que exigem segurança e confiabilidade. Sua sintaxe clara e concisa, ênfase na segurança e desempenho, e comunidade ativa e acolhedora tornaram-na uma linguagem moderna em constante crescimento, frequentemente utilizada em jogos, sistemas operacionais e outras aplicações de alto desempenho.

## Características Gerais

Rust é uma linguagem de programação de sistema que se destaca por sua ênfase em segurança e desempenho. Sua sintaxe clara e concisa, gerenciamento de memória seguro, verificação de limites de vetor em tempo de compilação e recursos de programação concorrente segura são algumas de suas características mais distintas. Além disso, Rust é uma linguagem multiplataforma e tem uma comunidade ativa, acolhedora e pronta para ajudar.

## Instalação

### **_Instalação no Windows_**:

1. Acesse o site oficial da Rust ([https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)) e clique no botão "Install Rust".

2. Selecione o instalador para Windows e execute-o.

3. Siga as instruções do instalador até que a instalação seja concluída.

### **_Instalação no Linux_**:

1. Abra o terminal e atualize os pacotes do sistema com o comando "sudo apt-get update" (Ubuntu/Debian) ou "sudo yum update" (Red Hat/Fedora).

2. Instale as dependências necessárias com o comando "sudo apt-get install build-essential" (Ubuntu/Debian) ou "sudo yum groupinstall 'Development Tools'" (Red Hat/Fedora).

3. Acesse o site oficial da Rust ([https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)) e copie o comando de instalação da versão mais recente do Rust para Linux.

4. Cole o comando no terminal e pressione Enter para iniciar o processo de instalação.

5. Aguarde até que a instalação seja concluída.

Após concluir a instalação, você pode testar se a linguagem Rust está funcionando corretamente executando o comando "rustc --version" no terminal. Ele deve exibir a versão atual da linguagem instalada.

Caso não queira instalar na sua máquina, é possível executar codigo online utilizando o [playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021) do próprio rust.

## Sintaxe

### **_Declaração de variáveis_**

Em Rust, as variáveis são declaradas usando a palavra-chave "let" seguida pelo nome da variável e do valor atribuído a ela. A declaração de variáveis em Rust é do tipo estático, o que significa que o tipo da variável deve ser especificado no momento da declaração.

Aqui está um exemplo de como declarar variáveis em Rust:

```rust

let nome=  "Filipe Zaidan";

let idade=  22;

let altura=  1.62;

```

Neste exemplo, três variáveis são declaradas: "nome", "idade" e "altura". A primeira variável armazena uma string, enquanto as outras duas armazenam números inteiros e decimais, respectivamente. Note que o tipo de dado de cada variável não é obrigatorio.

Rust é uma linguagem de programação fortemente tipada, o que significa que o tipo de dado deve ser especificado explicitamente. Além disso, as variáveis em Rust são sensíveis a maiúsculas e minúsculas, o que significa que "nome" e "Nome" são variáveis diferentes em Rust.

### **_Como realizar um Hello World_**

Para realizar um "Hello World" em Rust, podemos utilizar a função `println!`. Ela é usada para imprimir uma linha de texto no console e pode receber vários argumentos que serão impressos separados por vírgula.

```rust

fn  main() {

println!("Hello World");

}

```

Também podemos utilizar a macro "print!" que tem a mesma função da macro "println!" porém sem imprimir uma nova linha após a mensagem. O "print!" só pode receber um argumento por vez e retorna o valor 1 por padrão, o que o torna um pouco mais lento em relação ao "println!".

```rust

fn  main() {

print!("Hello World com print");

}

```

### **_Estruturas de Condição_**

Rust possui três estruturas de condição principais: `if`, `if-else`, e `if-elseif-else`.

A estrutura `if` é utilizada para testar uma condição simples, e executa um bloco de código se essa condição for verdadeira. Por exemplo:

### if:

```rust

fn  main() {

let x =  5;

if x <  10 {

println!("x é menor do que 10");

}

}

```

Se a condição `x < 10` for verdadeira, a mensagem "x é menor do que 10" será impressa no console.

### if-else:

A estrutura `if-else` é utilizada para testar uma condição e executar um bloco de código se a condição for verdadeira, e outro bloco se a condição for falsa. Por exemplo:

```rust

fn  main() {

let x =  5;

if x <  10{

println!("x é menor do que 10");

}else{

println!("x é maior ou igual a 10");

}

}

```

Se a condição `x < 10` for verdadeira, a mensagem "x é menor do que 10" será impressa no console. Se a condição for falsa, a mensagem "x é maior ou igual a 10" será impressa.

### if-elseif-else:

A estrutura `if-else if-else` é utilizada para testar múltiplas condições e executar diferentes blocos de código baseados nesses testes. Por exemplo:

```rust

fn  main() {

let x =  5;

if x <  0 {

println!("x é negativo");

}else  if x ==  0 {

println!("x é zero");

}else {

println!("x é positivo");

}

}

```

Se `x` for menor que zero, a mensagem `x é negativo` será impressa. Casp `x` for igual a zero, a mensagem `x é zero` será impressa. Ou ainda `x` for maior do que zero, a mensagem `x é positivo` será impressa.

#### Switch

Em Rust, não temos a construção `switch`, mas podemos alcançar um comportamento semelhante usando o bloco `match`. Aqui está um exemplo de como reescrever o código anterior em Rust:

```rust
fn main() {
    let numero = 5;

    match numero {
        1 => println!("O número é um"),
        2 => println!("O número é dois"),
        3 | 4 | 5 => println!("O número está entre 3 e 5"),
        6..=10 => println!("O número está entre 6 e 10"),
        _ => println!("O número não está entre 1 e 10"),
    }
}
```

Neste exemplo, temos uma variável `numero` que possui o valor 5. O bloco `match` verifica o valor da variável e executa o código correspondente ao padrão correspondente.

Neste caso, temos vários padrões: `1`, `2`, `3 | 4 | 5`, `6..=10` e `_`. O `_` representa um padrão curinga, que corresponde a qualquer valor que não corresponda aos padrões anteriores.

Ao executar este código, a saída será:

```rust
O número está entre 3 e 5
```

### **_Estruturas de Repetição_**

Em Rust, as estruturas de repetição permitem que um conjunto de instruções seja executado várias vezes com base em uma condição específica. Existem duas estruturas de repetição principais em Rust: `while` e `for`.

#### For

A estrutura `for` é usada quando se sabe quantas vezes um conjunto de instruções precisa ser repetido.

```rust
for variavel in inicializacao..condicao {
    // código a ser executado
}
```

O parâmetro `variavel` representa a variável de controle do loop, `inicializacao` é o valor inicial da variável, e `condicao` é a condição que determina se o loop continua ou não.

O corpo do loop, delimitado por chaves, contém o código a ser executado em cada iteração do loop. O código pode incluir qualquer número de instruções, incluindo outras estruturas de controle de fluxo.

Por exemplo:

```rust
fn main() {
    println!("======= Ciclo For =======");
    // Uma simples estrutura de repetição com for
    for i in 1..=10 {
        println!("Contador {}", i);
    }
}
```

#### while

A estrutura `while` é usada quando não se sabe quantas vezes um conjunto de instruções precisará ser repetido.

A condição é testada antes de cada iteração e, se for verdadeira, as instruções dentro do loop serão executadas.

O loop continuará até que a condição se torne falsa. A estrutura básica em Rust é a seguinte:

```rust
while condicao {
    // código a ser executado
}
```

Exemplo:

```rust
fn main() {
    // Inicializando a variável de controle
    let mut controle = 1;

    // Enquanto a variável de controle for menor ou igual a 10, o bloco de código será executado
    while controle <= 10 {
        println!("Contador {}", controle);
        // Incrementando a variável de controle
        controle += 1;
    }
}

```

### **_Modularização (Funções)_**

As funções são blocos de código que executam uma tarefa específica e podem ser reutilizadas em diferentes partes do programa.

#### Declaração de uma função

Em Rust, a declaração de uma função é feita usando a palavra-chave `fn`, seguida pelo nome da função e pelos parâmetros entre parênteses. Aqui está um exemplo de uma função simples que imprime uma mensagem na tela:

```rust
fn hello() {
    println!("Hello, World!");
}
```

Neste exemplo, declaramos a função `hello` sem nenhum parâmetro. A função imprime a mensagem "Hello, World!" usando a macro `println!`.

#### Chamada de uma função

Para chamar uma função em Rust, basta escrever o nome da função seguido por parênteses. Vamos chamar a função `hello` que declaramos anteriormente:

```rust
fn main() {
    hello(); // Chamada da função saudacao
}
```

A função `hello` será executada quando chamada dentro da função `main`.

#### Funções com parâmetros

As funções em Rust podem ter parâmetros, que são valores que podem ser passados para a função para que ela possa executar tarefas específicas. Vamos modificar nossa função `hello` para receber um nome como parâmetro e imprimir uma saudação personalizada:

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

```

Aqui, adicionamos um parâmetro `name` do tipo `&str`, que é uma referência a uma string. A função imprimirá a mensagem "Hello, [name]!", onde `[name]` será substituído pelo valor fornecido ao chamar a função.

#### Retorno de valores

Em Rust, as funções também podem retornar valores usando a palavra-chave `->` seguida pelo tipo do valor de retorno. Vamos criar uma função que retorna a soma de dois números inteiros:

```rust
fn soma(a: i32, b: i32) -> i32 {
    let resultado = a + b;
    resultado // O valor resultante será retornado implicitamente
}

```

A função `soma` recebe dois parâmetros `a` e `b` do tipo `i32` (inteiro de 32 bits) e retorna um valor do mesmo tipo. Neste exemplo, a soma dos dois números é atribuída à variável `resultado`, que é retornada implicitamente.
