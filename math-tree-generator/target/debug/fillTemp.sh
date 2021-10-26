#!/bin/bash

for((i=0;i<100;i++));do ./math-tree-generator |dot -Tpng > temp/out$i.png; done