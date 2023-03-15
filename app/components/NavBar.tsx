import { A } from "@solidjs/router";
import { JSX } from "solid-js";

type NavItemProps = {
  href: string;
  children: JSX.Element;
  end: boolean;
};

function NavItem({ href, end, children }: NavItemProps) {
  return (
    <li class="relative block px-3 py-2 transition">
      <A
        href={href}
        end={end}
        activeClass="text-teal-500"
        inactiveClass="hover:text-teal-500"
      >
        {children}
      </A>
    </li>
  );
}

export function NavBar() {
  return (
    <nav class="flex pt-4 pb-10">
      <ul class="flex rounded-full bg-white/90 px-3 text-sm font-medium text-zinc-800 shadow-lg shadow-zinc-800/5 ring-1 ring-zinc-900/5 backdrop-blur">
        <NavItem end={true} href="/">
          Home
        </NavItem>
        <NavItem end={true} href="/about">
          About
        </NavItem>
      </ul>
    </nav>
  );
}
