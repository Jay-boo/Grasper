from rest_framework import serializers
from .models import Messages, MessageTags, Tags

class TagSerializer(serializers.ModelSerializer):
    class Meta:
        model = Tags
        fields = ['label', 'theme']

class MessageTagSerializer(serializers.ModelSerializer):
    tag = TagSerializer(source='tagid')

    class Meta:
        model = MessageTags
        fields = ['tag']

class MessageSerializer(serializers.ModelSerializer):
    tags = serializers.SerializerMethodField()

    class Meta:
        model = Messages
        fields = ['date', 'content', 'title', 'link', 'tags']

    def get_tags(self, obj):
        request = self.context.get('request')
        theme = request.GET.get('theme', None)
        if theme:
            message_tags = MessageTags.objects.filter(messageid=obj, tagid__theme=theme)
        else:
            message_tags = MessageTags.objects.filter(messageid=obj)
        return MessageTagSerializer(message_tags, many=True).data

