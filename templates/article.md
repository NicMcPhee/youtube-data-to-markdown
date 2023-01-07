+++
title = {{ title }}
date = {{ date }}
description = "{{ description | clean_text | replace(from='"', to="'") }}"

[extra]
subject = {{ subject }}
playlist = {{ playlist_code }}
video_code = {{ code }}
+++

> This description was scraped from
> [the YouTube video page](https://www.youtube.com/watch?v={{ code | replace(from='"', to="") }}&list={{ playlist_code | replace(from='"', to="") }}).
> YouTube doesn't allow angle brackets, which are frequently used
> in Rust generics. To make the YouTube parser happy I replaced the
> angle brackets with parentheses when writing this description.
> So, yes, I know that a lot of the Rust snippets below are broken.
>
> In theory I should go through these and replace
> the appropriate parentheses with angle brackets, but I don't
> know if/when that will ever happen. Pull requests always
> welcome, though. :-)
>
> Thanks – Nic

<div>
{% raw %} {{ {% endraw %}
    youtube(id={{ code }}, playlist={{ playlist_code }}, class="flex grow")
{% raw %} }} {% endraw %}
</div>

{{ body | clean_text }}
