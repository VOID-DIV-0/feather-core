time
• time now into ::t
• time parse '2025-08-08T12:00:00Z' into ::t
• time add ::t days '7' into ::t2
• time diff ::a ::b in 'days' into @days
• time format ::t 'YYYY-MM-DD' into @s
• time compare ::a after|before|equal ::b into @ok
