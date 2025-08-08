record delete @rec. # remove record entirely (unset/clear)
record exists @rec into @ok. # check if record is bound and non-empty
record empty @rec into @ok. # check if value == ''
record length @rec into @len. # number of characters
record matches @rec 'regex' into @ok.
record is-numeric @rec into @ok.
record is-integer @rec into @ok.
record is-decimal @rec into @ok.
record is-date @rec 'YYYY-MM-DD' into @ok.
record lens @rec as integer into @out. # throws if invalid
record try-lens @rec as integer into @out ok @ok. # sets @ok true/false
record lens @rec as decimal into @out.
record lens @rec as json into ::container.
