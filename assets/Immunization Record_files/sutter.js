/** sutter.js
Sutter's custom Javascript file **/

/* Pop up new window */
function popUpWindow(url, name, attributes) {
    window.open(url, name, attributes);
}

function Set_Cookie(name, value, days) {
    if (days) {
        var date = new Date();
        date.setTime(date.getTime() + (days * 24 * 60 * 60 * 1000));
        var expires = "; expires=" + date.toGMTString();
	}
    else var expires = "";
    document.cookie = name + "=" + value + expires + "; path=/";
}

function Get_Cookie(name) {
    var start = document.cookie.indexOf(name + '=');
    var len = start + name.length + 1;
    if ((!start) && (name != document.cookie.substring(0, name.length)))
	return "";
    if (start == -1)
	return "";
    var end = document.cookie.indexOf(';', len);
    if (end == -1) end = document.cookie.length;
    return unescape(document.cookie.substring(len, end));
}


/*******************************************************************************
	setBackLinkHref(dest)
	By:   Don Zacharias
	Date: 2/26/2010
	Purpose: Set the "back" link's href to dest
	Note: If you use this in post text the back link hasn't been written, use addEvent
	Example: addEvent(window,'load',function(){setBackLinkHref("inside.asp?mode=myCustomHref")});
*******************************************************************************/
function setBackLinkHref(dest) {
    $(".back a.button").prop("href", dest);
}

/*return a date, otherwise return false*/
function createDate(str) {
    var parms = str.split(/[\.\-\/]/);
    var yyyy = parseInt(parms[2], 10);
    var dd = parseInt(parms[1], 10);
    var mm = parseInt(parms[0], 10);
    var date = new Date(yyyy, mm - 1, dd, 0, 0, 0, 0);
    if (mm === (date.getMonth() + 1) && dd === date.getDate() && yyyy === date.getFullYear())
	return date;
    else return false;
}

function getURLParameter(name) {
    return decodeURIComponent((RegExp(name + '=' + '(.+?)(&|$)').exec(location.search) || [, ""])[1]);
}

//handleEnterKey
// For form "frm" make sure that the button "defaultBtn" is the one
// that takes the action of the Enter key. Run this on page load.
// Author: Don Zacharias
// Date:  12/08/10
function handleEnterKey(frm, defaultBtn) {
    $(frm).keypress(function (e) {
        if ((e.which && e.which == 13) || (e.keyCode && e.keyCode == 13)) {
            $(defaultBtn).click();
            return false;
			} else {
            return true;
		}
	});
}

/* *************************************
	function setWindowTitle
	params: doc - the document element of the page
	title (options) - the new title
	purpose: Appends " - " to the document.title
	followed either by the title param or
	the text of ".title h2"
	author: Don Zacharias
	date:	2/23/11
****************************************/
function setWindowTitle(doc, title) {
    var newtitle = "";
    if (doc === undefined) {
        doc = document;
	}
    //add trailing dash unless it already ends with one
    if (doc.title.indexOf("-") == doc.title.length - 1) {
        newtitle = " ";
		} else {
        newtitle = " - ";
	}
    if (title !== undefined) {
        newtitle += title;
		} else if ($(".title h1", doc).text().length > 0) {
        newtitle += $(".title h1", doc).text();
	}
    doc.title += newtitle;
}

//Kevin Denison:
//Looks for a selector (where to look) within a set (element info) and
//changes the color of the parent element (table row) if it matches the filterString selector by adding a css class as defined by the color parameter.
function colorElementRow(elementInfo, whereToLook, color, filterSting) {
    $(whereToLook, elementInfo).filter(filterSting).parent().addClass(color);
}

//same as above except
//changes the color of the element found itself.
function colorElement(elementInfo, whereToLook, color, filterString) {
    $(whereToLook, elementInfo).filter(filterString).addClass(color);
}

//This function adds or removes the "*No Response Required*" text to the subject line of message replies from the patient
function updateSubject() {
    if ($("#subjectUpdateCheck").is(':checked')) {
        var subject = $('#subject').val();
        if (subject.indexOf("*No Response Required*") < 0) {
            $('#subject').val($('#subject').val() + " *No Response Required*");
            //append if checking the box
		}
		} else {
        var removeText = $('#subject').val();
        $('#subject').val(removeText.replace(" *No Response Required*", ""));
        //remove if not
	}
}

//launch popup window in them middle of the users' screen
function popUpWindowCenter(url, name, attributes) {
    //get window width & height
    var intWidth = $(window).width();
    var intHeight = $(window).height();
    //Get x and y coordinates to place window in center of the screen
    var xCenter = intWidth / 2;
    var yCenter = intHeight / 2;

    win = window.open(url, name, attributes);
    win.moveTo(xCenter, yCenter);
}

//Parse querystring
//Usage: page.html?mykey=value
//urlParams["mykey"] (should equal "value")
var urlParams = {};
(function () {
    var match,
	pl = /\+/g, // Regex for replacing addition symbol with a space
	search = /([^&=]+)=?([^&]*)/g,
	decode = function (s) {
		return decodeURIComponent(s.replace(pl, " "));
	},
	query = window.location.search.substring(1);

    while (match = search.exec(query))
	urlParams[decode(match[1])] = decode(match[2]);
})();

//return largest item in an array
// Array.prototype.max = function () { return Math.max.apply(Math, this) };
function getMaxOfArray(numArray) {
  return Math.max.apply(null, numArray);
}

//IE fix for location.origin
if (!window.location.origin) {
	window.location.origin = window.location.protocol + "//" + window.location.hostname + (window.location.port ? ':' + window.location.port: '');
}

// //Polyfill for array.includes
// if (!Array.prototype.includes) {
	// Array.prototype.includes = function(searchElement /*, fromIndex*/ ) {
		// 'use strict';
		// var O = Object(this);
		// var len = parseInt(O.length) || 0;
		// if (len === 0) {
			// return false;
		// }
		// var n = parseInt(arguments[1]) || 0;
		// var k;
		// if (n >= 0) {
			// k = n;
			// } else {
			// k = len + n;
			// if (k < 0) {k = 0;}
		// }
		// var currentElement;
		// while (k < len) {
			// currentElement = O[k];
			// if (searchElement === currentElement ||
			// (searchElement !== searchElement && currentElement !== currentElement)) { // NaN !== NaN
				// return true;
			// }
			// k++;
		// }
		// return false;
	// };
// }

function parseURL(url) {
    var parser = document.createElement('a'),
        searchObject = {},
        queries, split, i;
    // Let the browser do the work
    parser.href = url;
    // Convert query string to object
    queries = parser.search.replace(/^\?/, '').split('&');
    for( i = 0; i < queries.length; i++ ) {
        split = queries[i].split('=');
        searchObject[split[0]] = split[1];
    }
    return {
        protocol: parser.protocol,
        host: parser.host,
        hostname: parser.hostname,
        port: parser.port,
        pathname: parser.pathname,
        search: parser.search,
        searchObject: searchObject,
        hash: parser.hash
    };
}

function isBusinessHours(hour, day, hStart, hEnd, dStart, dEnd) {
    //if hour is between hStart and hEnd, and day is between dStart and dEnd
    return ((hStart <= hour && hour < hEnd) && (dStart <= day && day <= dEnd));
}

//All the pageload actions go here
document.addEventListener("DOMContentLoaded", function(event) {
    //jQuery-UI scripts
    if ($.fn.dialog !== undefined) {
        $(".dialog").dialog({ autoOpen: false, dialogClass: "sutter", minWidth: 420 });

        $(".dialog").prepend("<span class='hidden accessible'>start of dialog content</span>");
        $(".dialog").append("<span class='hidden accessible'>end of dialog content</span>");
        //dialog opener click action
        //Don Zacharias - 1/26/2012
        //When these links are clicked, open the dialog with the ID corresponding
        //to the HREF of the link, and cancel the default link action
        //requires jQuery, jQuery-UI
        $("a.dialogOpener").click(function (e) {
            $($(this).attr("href")).dialog("open");
            e.preventDefault();
        });
    }

    //$(document).on("click", "a", function(e) {
    $("a").each(function(e){
        //if href starts with http and does not include our website
        //or does contain some links that opedcn external URIs
        //cancel if link does not start with http
        if (this.href.indexOf("http") != 0) return;
        var re = new RegExp(/(.*)(sutterhealth|linkages)\.org/);
        var refpages = ["/mho/link.asp", "/mho/contentsearch.asp"];
        var url = parseURL(this.href);
        var site = url.protocol+"//"+url.host;
        var isrefpage = false;
        for (var i = 0; i <= refpages.length; i++) {
            if (url.pathname == refpages[i]) {
                isrefpage = true;        
            }
        }        
        if (site.indexOf(location.origin) === 0 && !isrefpage) return; 
        if (re.test(url.hostname)) return;
        //if we're here, pop up the alert      
        this.onclick = function(e){ 
            alert("The information contained in this website is for general information purposes only. The information is provided by Sutter Health and while we endeavor to keep the information up to date and correct, we make no representations or warranties of any kind, express or implied, about the completeness, accuracy, reliability, suitability or availability with respect to the website or the information, products, services, or related graphics contained on the website for any purpose. Any reliance you place on such information is therefore strictly at your own risk. Through this website you are able to link to other websites which are not under the control of Sutter Health. We have no control over the nature, content and availability of those sites. The inclusion of any links does not necessarily imply a recommendation or endorse the views expressed within them.") 
        };
    });

    $("#menu .findClinLink").click(function () {
        dcsMultiTrack("DCS.dcssip", "www.sutterhealth.org", "DCS.dcsuri", "/findadoctor/" + this.id, "WT.ti", "Find%20a%20Clinician (" + this.id + ")");
    });

    //track clicks for quick links
    $("#links a").click(function () {
        var link = this.innerText;
        dcsMultiTrack('WT.ti', link + ' [QUICKLINK]');
        //return;
	});


    //CRQ94714 add aria-expanded for menu group links
    $("#menu .mnutitle a").attr("aria-expanded", "false");

    //make questions same width 85927
    //Updated for Epic 2017 2316108
    $(".question, .questiongroupitem, .reasonList").not(".mobile .question, .mobile .questiongroupitem, .mobile .reasonList").each(function () {
        var ctr = this;
        var lblWidths = [];
        $("label", ctr).each(function () {
            lblWidths.push($(this).outerWidth());
		});
        //see above
        var lblWidest = getMaxOfArray(lblWidths);
        $("label", ctr).width(lblWidest);
	});

	//52100 Hide based on feature required
	$("[data-featurerequired]").each(function() {
		var that = this;
		var feat = $(that).attr("data-featurerequired");
		$.get("/mho/checkfeature.asp", { feature: feat }, function(val) {
			if (val == 0) {
				$(that).hide();
			}
		});
	});
	    
    //Custom themeing stuff
    $("#breadcrumbs").insertAfter("#header");
    var ti = $(".title h1").text();
    $("#breadcrumbs ol").append("<li>"+ti+"</li>");
    $("#ecosystem-footer").insertAfter("#footer");
    $("#breadcrumbs, #ecosystem-footer").removeClass("hidden");
    $("#chatCtrls").appendTo("#account");

    //menu list pushed over the same width as the logout button
    $(".ecosystem-menu-list ul").css("right", $(".ecosystem-logout").outerWidth());
    
    $(".ecosystem-menu .menu-opener").on("click",function() {
        var opener = this;
        $(opener).toggleClass("is-active");
        if ($(opener).hasClass("is-active")) {
            $(".ecosystem-menu-list ul").show();
            $(document).on("click", function(e) {
                //if they clicked elsewhere
                if (opener != e.target) {
                    $(opener).toggleClass("is-active");
                    $(".ecosystem-menu-list ul").hide();
                    $(document).off("click");
                }
            });
        } else {
            $(".ecosystem-menu-list ul").hide();
            $(document).off("click");
        }
    });
    $.ajax({url:"/mho/getdata.asp", data:{k: "loginname"}}).done(function(loginname){ 
		if (loginname!="") {
			$("#ecosystem-banner .menu-opener").text("Welcome, " + loginname);
			var menuWidth = $(".ecosystem-menu .menu-opener").outerWidth();    
			$(".ecosystem-menu-list").width(menuWidth + 3); //room for border        
		}        
    });
    $.ajax({url:"/mho/getdata.asp", data:{k: "svcaname"}}).done(function(svcaname){ 
		if (svcaname!="") {
			$("#ecosystem-banner .svcaname").text(svcaname);
		}        
    });					
    $("#localeswitch").prependTo(".ecosystem-menu");        
																										
    //Chat button
    var now, hour, day;		
    now = moment().tz('America/Los_Angeles');
    hour = now.format('H');
    day = now.format('d');
    if (!isBusinessHours(hour, day, 7, 19, 1, 5)) {
        $("#chatNowBtn").hide();
        $("#chatOffline").show();						
    }			
	
	$("#chatCtrls").removeClass("hidden");   

	$(".moveToSidebar").appendTo("#sidebar");

	$(".back a").on("click",function(){window.location.replace(this.href)});
        
	$("#moveToMessagingSidebar").appendTo(".md_review_index #sidebar .quickLinks").removeClass("hidden");

    $(".md_visitdetails_index #commprefslink").insertAfter(".updatewaitlistlink");    

    $(document).on("keyup", "#EditTokenPopUp #EditNicknameField", function(){
        // console.log(this.value);
        var digitsRe = /(\d\D*){9,}/;  //contains a digit followed by optional non-digit repeated 9 or more times
        if (digitsRe.test(this.value)) { 
            $(".button.editNicknameSave").addClass("disabled").attr("disabled","disabled");
            alert("'Nickname' field cannot contain more than 8 numbers.");            
        } else {
            $(".button.editNicknameSave").removeClass("disabled").attr("disabled",false);
        }         
    });	
	
    $(document).on("click", ".editButton", function() {
        setTimeout(function() { 
			$("#ContactInformation_Email, #OtherInformation_1").addClass("disabled").attr("readonly","readonly");
        }, 200);
    });
	
    //on the OMW page if dispGroups is passed
    if (document.body.classList.contains("md_onmyway_index") && urlParams["dispGroups"] != null) {
        var mapLoadHandler = function(event) { 
            var ctr = document.querySelector("#search_coordinates_container");
            if (ctr) ctr.innerHTML = "<a href='/mho/scheduling/onmyway'>Show all locations</a>";
            if (event.data.message == "map_load") {
                var listRows = document.querySelectorAll("#department_map_container .listRow");
                //if there is one listRow displayed, click on it
                if (listRows.length == 1) {
                    listRows[0].click();
                    //only need to do this once, so remove the handler
                    window.removeEventListener("message", mapLoadHandler);
                }
            }
			
			//If rfvID is passed
			if(urlParams["rfvID"] != null) {
				var $allLocations = $("#search_coordinates_container a");
				var updateURL = "";
				
				if($allLocations.length == 1) {
					if(/\?/.test($allLocations.attr("href"))) {
						updateURL = $allLocations.attr("href") + "&rfvID=" + urlParams["rfvID"];
					} else {
						updateURL = $allLocations.attr("href") + "?rfvID=" + urlParams["rfvID"];
					}

					$allLocations.attr("href", updateURL);
				}
			}
        };

        //attach the above function as a handler
        window.addEventListener("message", mapLoadHandler);
    }
	
	// MHOS19-14 - Remediate accessibility concerns with MHO public forms
	if(document.body.classList.contains("md_forgotpasswd") || document.body.classList.contains("md_passwdreset") || document.body.classList.contains("md_passwdreQ")) {
		//Add link text to header logo (ADA)
		$("#main a.logo").append("<span class='clearlabel'>My Health Online</span>");
	}
	
	// Scheduling: Mammography Screening
	// Save responses from Mammography Screening questionnaire as cookies. Pass to Mammography Request Form
	if(document.body.classList.contains("md_scheduling_index")) {
		$(document).on("mouseenter", "#next-step3", function() {
			if($("span[data-sh-lql-id]").length) {
				var $checkedInputs = $("#qnr-form3 input[type='radio']:checked");

				$.each($checkedInputs, function(key) {
					document.cookie = "lql-cookie-" + $("span[data-sh-lql-id]:eq("+key+")").attr("data-sh-lql-id") + "=" + $(this).val() + ";path=/";
				});
			}
		});
	}
	
	// MHOE19-6 CR3756 Request Appointment with New Clinician
	// Add warning content to scheduling step 1, desktop and mobile
	if(document.body.classList.contains("md_scheduling_index")) {
		$(".warning").addClass("hidden");
		
		$(".title").after($(".warning"));
		$(".warning").removeClass("hidden");

		$("#scheduling-workflow").on("click", ".cardlist .card, .workflowselect a", function() {
			if($(".warning").length) {
				$(".warning").remove();
			}
		});
	}

	// MHOE19-21: Clinical Message routing redesign
	if(document.body.classList.contains("md_custsvc") && window.location.href.match(/\#Insurance(\%20| )?Questions/)) {
		var $languageToggle = $("#localeswitch a");
		var updateURL = $languageToggle.attr("href") + "#Insurance%20Questions";
		$languageToggle.attr("href", updateURL);
	}
	
	// MHOS19-22 - Online Enrollment - Required Fields
	if(document.body.classList.contains("md_onmyway_postlogin") || document.body.classList.contains("md_onmyway_index")) {
		$(document).on("click", ".listHolder li.listRow", function() {
			var reasonForVisit = setInterval(function() { 
				if($("#reasonForVisitCommentLabel").length) {
					$("#reasonForVisitCommentLabel").addClass("required");
					clearInterval(reasonForVisit);
				}
			}, 1000);
		});
	}
	
	// MHOE19-26 - Preference Page Adjustment - Hide preferences that can not be adjusted
	if(document.body.classList.contains("md_communications_manage")) {
		var $optionsGroup = $("div.options");
		var $parentGroup = $("div.group");

		$.each($optionsGroup, function() {
			//Count of <a> tags in each options group
			var $optionsCount = $(this).find("a.mediabutton").length;
			
			//Count of <a> tags that are locked
			var $optionsLocked = $(this).find("a.locked").length;
			
			//If all a tags in this group are locked then remove this row
			if($optionsCount == $optionsLocked) {
				$(this).parents("li.tickleritem").remove();
			}
		});

		$.each($parentGroup, function() {
			//If parent group doesn't include any tickleritem rows then remove
			if($(this).find("li.tickleritem").length < 1) {
				$(this).remove();
			}
		});
		
		//Hide remaining locked icons and count <span> below each parent group
		$optionsGroup.find("a.locked").addClass("hidden");
		$(".mediacount").addClass("hidden");
	}
	
	// ITSM-698 - MHO Login Enhancement: Known Account Reclamation
	if(document.body.classList.contains("md_standalone_signup") && document.body.classList.contains("isPrelogin")) {
		if(urlParams["email"] != null) {
			var userEmail = urlParams["email"];
			
			$("#Email").val(userEmail);
			$("#EmailVerification").val(userEmail);
		}
	}
});