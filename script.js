function isVisible(element) {        
    var viewportTop = $('html').scrollTop();
    var viewportBottom = viewportTop + $(window).height();
    
    var elementTop = Math.round($(element).offset().top);
    var elementBottom = elementTop + $(element).height();

    return ((elementTop < viewportBottom) && (elementBottom < viewportBottom));
  }

  function triggerAnimations() {
    var $elements = $('.section');

    $elements.each(function(index, element) {
      if ($(element).children('.section-content').hasClass('fade-in')) {
        return;
      } 
      else if (isVisible(element)) {
        $(element).children('.section-title').addClass('fade-right');
        $(element).children('.section-content').addClass('fade-in');
      }
    });

  }

  $(window).scroll(function(){
    triggerAnimations();
  });