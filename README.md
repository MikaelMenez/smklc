# SMKL Transpilador

Transpilador em **Rust** que converte a linguagem de marcação **SMKL** para **HTML**, oferecendo uma sintaxe mais limpa e legível.

## Proposta

Simplificar a escrita de HTML utilizando uma sintaxe baseada em chaves `{}` para hierarquia e parênteses para atributos, eliminando tags de fechamento repetitivas.

## Exemplo

**SMKL:**

```html
<span style="color: #ff7b72;">div</span><span style="color: #d2a8ff;">(class</span><span style="color: #ff7b72;">=</span><span style="color: #a5d6ff;">"container"</span><span style="color: #d2a8ff;">)</span><span style="color: #f0f6fc;">{</span>
    <span style="color: #7ee787;">"Título"</span>
<span style="color: #f0f6fc;">}</span>

```

**HTML:**

```html
<div class="container" id="principal">
    <h1>Título</h1>
    <br>
    <img src="imagem.png" alt="Exemplo">
</div>

```
