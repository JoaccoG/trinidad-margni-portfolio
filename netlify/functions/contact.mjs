const EMAIL_RE = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
const MAX_EMAIL_LEN = 320;
const MAX_MESSAGE_LEN = 5000;

const json = (body, status = 200) =>
  new Response(JSON.stringify(body), {
    status,
    headers: { "Content-Type": "application/json" },
  });

const escapeHtml = (str) =>
  str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");

export default async (req) => {
  if (req.method !== "POST") return json({ error: "Method not allowed" }, 405);

  const apiKey = Netlify.env.get("RESEND__API_KEY");
  const from = Netlify.env.get("EMAILS__FROM");
  const to = Netlify.env.get("EMAILS__TO");

  if (!apiKey || !from || !to)
    return json({ error: "Server misconfigured" }, 500);

  let body;
  try {
    body = await req.json();
  } catch {
    return json({ error: "Invalid JSON" }, 400);
  }

  if (body._gotcha) return json({ ok: true });

  const { email, message } = body;

  if (!email || typeof email !== "string" || email.length > MAX_EMAIL_LEN || !EMAIL_RE.test(email.trim()))
    return json({ error: "Valid email is required" }, 400);

  if (!message || typeof message !== "string" || message.trim().length === 0 || message.length > MAX_MESSAGE_LEN)
    return json({ error: `Message is required (max ${MAX_MESSAGE_LEN} characters)` }, 400);

  const safeEmail = escapeHtml(email.trim());
  const safeMessage = escapeHtml(message.trim());

  const html = `
    <div style="font-family:sans-serif;max-width:560px;margin:0 auto;padding:24px">
      <h2 style="font-size:18px;margin:0 0 16px">New message from your portfolio</h2>
      <p style="margin:0 0 8px"><strong>From:</strong> ${safeEmail}</p>
      <div style="background:#f5f5f5;padding:16px;border-radius:4px;white-space:pre-wrap;font-size:14px;line-height:1.6">${safeMessage}</div>
      <hr style="border:none;border-top:1px solid #e0e0e0;margin:24px 0 12px" />
      <p style="font-size:12px;color:#888;margin:0">Sent from trinidadmargni.com contact form</p>
    </div>`;

  try {
    const res = await fetch("https://api.resend.com/emails", {
      method: "POST",
      headers: {
        Authorization: `Bearer ${apiKey}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        from,
        to: [to],
        reply_to: [email.trim()],
        subject: `New portfolio message from ${email.trim()}`,
        html,
      }),
    });

    if (!res.ok) {
      const err = await res.text();
      console.error("Resend error:", res.status, err);
      return json({ error: "Failed to send message" }, 500);
    }

    return json({ ok: true });
  } catch (err) {
    console.error("Resend fetch error:", err);
    return json({ error: "Failed to send message" }, 500);
  }
};
