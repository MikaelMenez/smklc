# SMKL Transpilador

Transpilador em **Rust** que converte a linguagem de marcação **SMKL** para **HTML**, oferecendo uma sintaxe mais limpa e legível.

## Proposta

Simplificar a escrita de HTML utilizando uma sintaxe baseada em chaves `{}` para hierarquia e parênteses para atributos, eliminando tags de fechamento repetitivas.

## Exemplo

**SMKL:**
```css
div(class="container", id="principal"){
    h1{ "Título" }
    |br|
    |img(src="imagem.png", alt="Exemplo")|
}
```
se traduz para :


**HTML**
```html
<div class="container" id="principal">
    <h1>Título</h1>
    <br>
    <img src="imagem.png" alt="Exemplo">
</div>
```
