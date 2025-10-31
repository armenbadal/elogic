# Տրամաբանական սխեմաների լեզու եւ սիմուլյատոր

> Այս նախագիծն օգտագործում եմ [Rust](https://www.rust-lang.org/) լեզուն 
  ուսումնասիրելու և սովորելու համար։

Լեզուն հնարավորություն է տալիս նկարագրել տրամաբանական սխեմաներ, որոնք
մշակվում և ներկայացվում են __NAND__ տարրերի տեսքով։ Համակարգը պետք է 
թույլ տա նաև հաշվարկել տրամաբանական սխեմայի արժեքը՝ տրված մուտքային 
արժեքների համար։

## Լեզվի քերականությունը

```
Design = { Schematic NewLines }.
Schematic = 'define' IDENT IdentList '->' IdentList NewLines
           { Instruction NewLines } 'end'.
IdentList = IDENT { IDENT }.
Instruction = IDENT SignalList -> IdentList.
SignalList = Signal { Signal }.
Signal = IDENT | '0' | '1' | 'true' | 'false'.
```

## Հրամանային տողը

```bash
$ elogic simulate <file> <schematic> <data>
```
