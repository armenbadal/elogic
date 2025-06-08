# Տրամաբանական սխեմաների լեզու եւ սիմուլյատոր

## Լեզվի քերականությունը

```
Design = { Scheme NewLines }.
Scheme = 'scheme' IDENT IdentList '->' IdentList NewLines
           { Instruction NewLines } 'end'.
IdentList = IDENT { IDENT }.
Instruction = IDENT SignalList -> IdentList.
SignalList = Signal { Signal }.
Signal = IDENT | '0' | '1' | 'true' | 'false'.
```

