+++
title = {{ title }}
date = {{ date }}
description = {{ description | restore_angle_brackets }}

[extra]
subject = {{ subject }}
playlist = {{ playlist_code }}
video_code = {{ code }}
+++

{% raw %}
<div class="flex">
    {{ youtube(id={{ video_code }}, playlist={{ playlist_code }}, class="grow") }}
</div>
{% endraw %}

{{ body | restore_angle_brackets }}
