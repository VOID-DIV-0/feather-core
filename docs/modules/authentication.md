authentication basic user @u pass @p into @header.
authentication bearer @token into @header.
authentication jwt parse @token into ::claims. // no verification yet
authentication jwt verify @token alg 'HS256' key @secret into @ok.
authentication jwt sign ::claims alg 'HS256' key @secret into @token.
