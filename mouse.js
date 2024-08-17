/**
 * Get mouse position.
 *
 * Ref: https://stackoverflow.com/questions/14924543/how-do-i-change-the-color-of-a-div-with-onclick-by-calling-a-function-in-javascr
 *
 * @param {Event} e
 */
function on_mouse_move(e, obj) {
  var m_posx = 0;
  var m_posy = 0;
  var e_posx = 0;
  var e_posy = 0;

  if (!e) {
    e = window.event;
  }

  if (e.pageX || e.pageY) {
    m_posx = e.pageX;
    m_posy = e.pageY;
  } else if (e.clientX || e.clientY) {
    m_posx =
      e.clientX +
      document.body.scrollLeft +
      document.documentElement.scrollLeft;
    m_posy =
      e.clientY + document.body.scrollTop + document.documentElement.scrollTop;
  }
  //get parent element position in document
  if (obj.offsetParent) {
    do {
      e_posx += obj.offsetLeft;
      e_posy += obj.offsetTop;
    } while ((obj = obj.offsetParent));
  }

  // mouse position minus elm position is mouseposition relative to element:
  // console.log(
  //   " X Position: " + (m_posx - e_posx) + " Y Position: " + (m_posy - e_posy)
  // );

  return [m_posx - e_posx, m_posy - e_posy];
}
