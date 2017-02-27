display dialog "Heads or Tails?" default answer ""
set answer to text returned of result

if answer starts with "h" and (random number 1) as boolean then
	display dialog "You were right!"
else
	display dialog "You were wrong.."
end if
