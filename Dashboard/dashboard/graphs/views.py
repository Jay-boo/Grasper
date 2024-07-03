from django.shortcuts import render, get_object_or_404
from .serializers import MessageSerializer
from .models import Topics, Messages, MessageTags
from django.db.models import Count
from django.http import JsonResponse, HttpResponseServerError
from django.contrib.auth.decorators import login_required
from datetime import datetime, timedelta
from django.db.models import Q

def home(request):
    try:
        response = messages_with_tags(request)
        if response.status_code != 200:
            raise Exception("Sorry, the database is not up.")
        else:
           return render(request, 'graphs/home.html')
    except Exception as e:
        return HttpResponseServerError(render(request, 'graphs/error.html', {'error': str(e)}))


def messages_by_topic(request):
    try:
        data = Messages.objects.values('topic__name') \
            .annotate(count_items=Count('id')) \
            .order_by('topic__name')
        return JsonResponse(list(data), safe=False)
    except Exception as e:
        return HttpResponseServerError(render(request, 'graphs/error.html', {'error': str(e)}))

# @login_required
def messages_by_tag(request):
    try:
        data = MessageTags.objects.values('tagid__label', 'tagid__theme').annotate(count_items=Count('messageid')).order_by('tagid__label')
        return JsonResponse(list(data), safe=False)
    except Exception as e:
        return HttpResponseServerError(render(request, 'graphs/error.html', {'error': str(e)}))

# @login_required
def messages_with_tags(request):
    try:
        period = request.GET.get('period', 'all')
        theme = request.GET.get('theme', None)

        if period == 'last_7_days':
            start_date = datetime.now() - timedelta(days=7)
            messages = Messages.objects.filter(date__gte=start_date)
        elif period == 'last_month':
            start_date = datetime.now() - timedelta(days=30)
            messages = Messages.objects.filter(date__gte=start_date)
        elif period == 'last_3_months':
            start_date = datetime.now() - timedelta(days=90)
            messages = Messages.objects.filter(date__gte=start_date)
        elif period == 'last_6_months':
            start_date = datetime.now() - timedelta(days=180)
            messages = Messages.objects.filter(date__gte=start_date)
        else:
            messages = Messages.objects.all()

        if theme:
            messages = messages.filter(
                Q(messagetags__tagid__theme=theme)
            ).distinct()

        serializer = MessageSerializer(messages, many=True, context={'request': request})
        return JsonResponse(serializer.data, safe=False)
    except Exception as e:
        return HttpResponseServerError(render(request, 'graphs/error.html', {'error': str(e)}))
