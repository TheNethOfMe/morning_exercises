const chatForm = document.getElementById("chat-form");
const chatMessages = document.querySelector(".chat-messages");
const roomNameDiv = document.getElementById("room-name");
const userList = document.getElementById("users");

// get username and room from url
const { username, room } = Qs.parse(location.search, {
  ignoreQueryPrefix: true
});

const socket = io();

// Join room
socket.emit("joinRoom", { username, room });

// Get room and users
socket.on("roomusers", ({ room, users }) => {
  outputRoomName(room);
  outputRoomUsers(users);
});

// Message from server
socket.on("message", message => {
  console.log(message);
  outputMessage(message);

  // Scroll down
  chatMessages.scrollTop = chatMessages.scrollHeight;
});

// Message submit
chatForm.addEventListener("submit", e => {
  e.preventDefault();
  const msg = e.target.elements.msg.value;

  // emiting a message to the server
  socket.emit("chatMessage", msg);

  // clear input
  e.target.elements.msg.value = "";
  e.target.elements.msg.focus();
});

// Output data to DOM
function outputMessage(msg) {
  const div = document.createElement("div");
  div.classList.add("message");
  div.innerHTML = `<p class="meta">${msg.username} <span>${msg.time}</span></p><p class="text">${msg.text}</p>`;
  document.querySelector(".chat-messages").appendChild(div);
}

function outputRoomName(room) {
  roomNameDiv.innerText = room;
}

function outputRoomUsers(users) {
  userList.innerHTML = `${users
    .map(user => `<li>${user.username}</li>`)
    .join("")}`;
}
