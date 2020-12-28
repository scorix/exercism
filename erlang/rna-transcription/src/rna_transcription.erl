-module(rna_transcription).

-export([to_rna/1]).

to_rna([]) -> [];
to_rna(String) -> transform(String, []).

transform([H|T], L) -> transform(T, L++rna([H]));
transform([], L) -> L.

rna("C") -> "G";
rna("G") -> "C";
rna("T") -> "A";
rna("A") -> "U";
rna(_) -> "".
