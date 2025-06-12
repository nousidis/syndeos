"use client"

import * as React from "react"
import {type Icon} from "@tabler/icons-react"

import {
    SidebarGroup,
    SidebarGroupContent,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from "@/components/ui/sidebar"
import {usePageContext} from "@/components/providers/page";

export function NavSecondary({items, ...props}: {
    items: {
        title: string
        url: string
        icon: Icon
    }[]
} & React.ComponentPropsWithoutRef<typeof SidebarGroup>) {
    const {setCurrentPage} = usePageContext();

    const handleClick = (item: { title: string; url: string }) => {
        if (item.title === 'Settings') {
            setCurrentPage('settings');
        }
    };

    return (
        <SidebarGroup {...props}>
            <SidebarGroupContent>
                <SidebarMenu>
                    {items.map((item) => (
                        <SidebarMenuItem key={item.title}>
                            <SidebarMenuButton
                                className={item.title !== 'Github' ? 'cursor-pointer': undefined}
                                asChild={item.title === 'Github'}
                                onClick={() => handleClick(item)}
                            >
                                {item.title === 'Github' ? (
                                    <a href={item.url} target="_blank" rel="noopener noreferrer">
                                        <item.icon/>
                                        <span>{item.title}</span>
                                    </a>
                                ) : (
                                    <>
                                        <item.icon/>
                                        <span>{item.title}</span>
                                    </>
                                )}
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    ))}
                    <SidebarMenuItem className="flex justify-items-end items-end">
                        <p className="text-muted-foreground font-semibold">v1.2.3</p>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarGroupContent>
        </SidebarGroup>
    )
}