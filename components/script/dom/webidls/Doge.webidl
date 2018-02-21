// https://jeenalee.github.io/doge-standard/

typedef sequence<DOMString> DogeInit;

[Constructor(optional DogeInit init),
 Exposed=(Window,Worker)]

interface Doge {
  void append(DOMString word);
  [Throws] DOMString random();
  [Throws] void remove(DOMString word);
  void printAll();
};