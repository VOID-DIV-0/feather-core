auth basic user @u pass @p into @header.
auth bearer @token into @header.
auth jwt parse @token into ::claims.     // no verification yet
auth jwt verify @token alg 'HS256' key @secret into @ok.
auth jwt sign ::claims alg 'HS256' key @secret into @token.