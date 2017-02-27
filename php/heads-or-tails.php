<?php

$fp = fopen('php://stdin', 'r');

while (true) {
  print "Heads or Tails?\n";

  $line = fgets($fp, 1024);
  $heads = rand(0, 1) > 0;

  if ($heads && substr($line, 0, 1) == "h") {
    print "You were right!\n";
  } else {
    print "You were wrong..\n";
  }

}
