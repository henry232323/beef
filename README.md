# compiler

### Todo
- Convert AST to some sort of "bytecode"
- Eval/exec function to run bytecode
- Parsing
  - Loops
  - Other operators
  - Import
  - Try/Catch
  - Break keyword
  - Continue keyword
  - Return keyword

### Bytecode Spec
| Modification<br/>Timestamp<br/>4 bytes | Description |
|----------------------------------------|-------------|
| Header                                 | Title       |
| Paragraph                              | Text        |

- Top level vars
```
dump   = header proto+ 0U
header = ESC 'L' 'J' versionB flagsU [namelenU nameB*]
proto  = lengthU pdata
pdata  = phead bcinsW* uvdataH* kgc* knum* [debugB*]
phead  = flagsB numparamsB framesizeB numuvB numkgcU numknU numbcU
[debuglenU [firstlineU numlineU]]
kgc    = kgctypeU { ktab | (loU hiU) | (rloU rhiU iloU ihiU) | strB* }
knum   = intU0 | (loU1 hiU)
ktab   = narrayU nhashU karray* khash*
karray = ktabk
khash  = ktabk ktabk
ktabk  = ktabtypeU { intU | (loU hiU) | strB* }
```